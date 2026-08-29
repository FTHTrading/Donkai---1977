//! Deterministic CIDv1 computation + blocking Kubo local RPC pin client.
//!
//! CIDv1 layout: `<version_varint><codec_varint><multihash>`, base32-lowercase-no-pad,
//! multibase 'b' prefix. Supports raw codec (0x55) and dag-pb codec (0x70)
//! wrapping a single-block UnixFS File PBNode.
//!
//! Kubo default RPC endpoint: `http://127.0.0.1:5001/api/v0`

use sha2::{Digest, Sha256};

const MULTIBASE_BASE32_LOWER: char = 'b';
const CID_VERSION_1: u8 = 0x01;
const CODEC_RAW: u8 = 0x55;
const CODEC_DAG_PB: u8 = 0x70;
const MULTIHASH_SHA2_256: u8 = 0x12;
const SHA256_DIGEST_LEN: u8 = 0x20;

pub const DEFAULT_KUBO_API: &str = "http://127.0.0.1:5001/api/v0";

/// Compute a raw-codec CIDv1 (`bafkrei…`) of arbitrary bytes.
pub fn compute_raw_cidv1(payload: &[u8]) -> String {
    let digest = Sha256::digest(payload);
    let mut cid_bytes = Vec::with_capacity(4 + 32);
    cid_bytes.push(CID_VERSION_1);
    cid_bytes.push(CODEC_RAW);
    cid_bytes.push(MULTIHASH_SHA2_256);
    cid_bytes.push(SHA256_DIGEST_LEN);
    cid_bytes.extend_from_slice(&digest);
    encode_multibase_base32_lower(&cid_bytes)
}

/// Compute a dag-pb-codec CIDv1 (`bafybei…`) wrapping `payload` as a single-block UnixFS File.
pub fn compute_dagpb_cidv1(payload: &[u8]) -> String {
    let pbnode = encode_unixfs_file_pbnode(payload);
    let digest = Sha256::digest(&pbnode);
    let mut cid_bytes = Vec::with_capacity(4 + 32);
    cid_bytes.push(CID_VERSION_1);
    cid_bytes.push(CODEC_DAG_PB);
    cid_bytes.push(MULTIHASH_SHA2_256);
    cid_bytes.push(SHA256_DIGEST_LEN);
    cid_bytes.extend_from_slice(&digest);
    encode_multibase_base32_lower(&cid_bytes)
}

// ---- Kubo RPC ----

#[derive(Debug, thiserror::Error)]
pub enum PinError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("kubo returned {0}: {1}")]
    Kubo(u16, String),
    #[error("could not parse kubo response: {0}")]
    Parse(String),
}

/// Pin `payload` to the local Kubo daemon at `DEFAULT_KUBO_API` and return the reported CID.
pub fn pin_to_kubo(payload: &[u8]) -> Result<String, PinError> {
    pin_to_kubo_at(DEFAULT_KUBO_API, payload)
}

/// Pin `payload` to any Kubo RPC endpoint. Uses `POST /add?cid-version=1&pin=true`.
pub fn pin_to_kubo_at(api_base: &str, payload: &[u8]) -> Result<String, PinError> {
    let url = format!(
        "{}/add?cid-version=1&pin=true",
        api_base.trim_end_matches('/')
    );
    let part =
        reqwest::blocking::multipart::Part::bytes(payload.to_vec()).file_name("donkai-payload");
    let form = reqwest::blocking::multipart::Form::new().part("file", part);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(&url).multipart(form).send()?;
    let status = resp.status();
    let body = resp.text()?;
    if !status.is_success() {
        return Err(PinError::Kubo(status.as_u16(), body));
    }
    let key = "\"Hash\":\"";
    let start = body
        .rfind(key)
        .ok_or_else(|| PinError::Parse("no \"Hash\" field in Kubo response".into()))?
        + key.len();
    let end_rel = body[start..]
        .find('"')
        .ok_or_else(|| PinError::Parse("unterminated \"Hash\" string".into()))?;
    Ok(body[start..start + end_rel].to_string())
}

// ---- internals ----

fn write_varint(buf: &mut Vec<u8>, mut val: u64) {
    while val >= 0x80 {
        buf.push((val as u8) | 0x80);
        val >>= 7;
    }
    buf.push(val as u8);
}

/// UnixFS single-block File PBNode.
///
/// PBNode: `field 1 (Data, bytes) = <unixfs bytes>`
/// UnixFS Data: `field 1 (Type, varint) = 2 (File)`, `field 2 (Data, bytes) = payload`,
///              `field 3 (filesize, varint) = payload.len()`.
fn encode_unixfs_file_pbnode(payload: &[u8]) -> Vec<u8> {
    let mut unixfs = Vec::with_capacity(payload.len() + 16);
    // Type = File (2)
    unixfs.push(0x08);
    unixfs.push(2);
    // Data
    unixfs.push(0x12);
    write_varint(&mut unixfs, payload.len() as u64);
    unixfs.extend_from_slice(payload);
    // filesize
    unixfs.push(0x18);
    write_varint(&mut unixfs, payload.len() as u64);

    let mut pbnode = Vec::with_capacity(unixfs.len() + 8);
    // PBNode.Data
    pbnode.push(0x0A);
    write_varint(&mut pbnode, unixfs.len() as u64);
    pbnode.extend_from_slice(&unixfs);
    pbnode
}

fn encode_multibase_base32_lower(bytes: &[u8]) -> String {
    let upper = data_encoding::BASE32_NOPAD.encode(bytes);
    let mut out = String::with_capacity(1 + upper.len());
    out.push(MULTIBASE_BASE32_LOWER);
    for ch in upper.chars() {
        out.push(ch.to_ascii_lowercase());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_cid_is_deterministic() {
        let a = compute_raw_cidv1(b"hello");
        let b = compute_raw_cidv1(b"hello");
        assert_eq!(a, b);
    }

    #[test]
    fn raw_cid_has_bafkrei_prefix() {
        let cid = compute_raw_cidv1(b"hello world");
        assert!(cid.starts_with("bafkrei"), "got {}", cid);
        assert_eq!(cid.len(), 59, "unexpected CID length: {}", cid);
    }

    #[test]
    fn dagpb_cid_has_bafybei_prefix() {
        let cid = compute_dagpb_cidv1(b"hello world");
        assert!(cid.starts_with("bafybei"), "got {}", cid);
        assert_eq!(cid.len(), 59, "unexpected CID length: {}", cid);
    }

    #[test]
    fn raw_and_dagpb_differ_for_same_payload() {
        let raw = compute_raw_cidv1(b"hello world");
        let dagpb = compute_dagpb_cidv1(b"hello world");
        assert_ne!(raw, dagpb);
    }

    #[test]
    fn different_payloads_yield_different_cids() {
        assert_ne!(compute_raw_cidv1(b"a"), compute_raw_cidv1(b"b"));
        assert_ne!(compute_dagpb_cidv1(b"a"), compute_dagpb_cidv1(b"b"));
    }

    #[test]
    fn varint_encoding_is_correct() {
        fn v(x: u64) -> Vec<u8> {
            let mut b = Vec::new();
            write_varint(&mut b, x);
            b
        }
        assert_eq!(v(0), vec![0]);
        assert_eq!(v(127), vec![127]);
        assert_eq!(v(128), vec![0x80, 0x01]);
        assert_eq!(v(300), vec![0xAC, 0x02]);
        assert_eq!(v(16_384), vec![0x80, 0x80, 0x01]);
    }

    #[test]
    fn empty_payload_still_produces_valid_cid() {
        let raw = compute_raw_cidv1(b"");
        let dp = compute_dagpb_cidv1(b"");
        assert!(raw.starts_with("bafk"));
        assert!(dp.starts_with("bafy"));
    }
}
