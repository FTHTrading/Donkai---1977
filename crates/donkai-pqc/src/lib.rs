//! Post-quantum crypto wrappers around `pqcrypto-mldsa` (FIPS 204 ML-DSA-87)
//! and `pqcrypto-mlkem` (FIPS 203 ML-KEM-1024).
//!
//! These primitives are NIST-standardized; this crate does not claim FIPS 140-3
//! module certification. `pqcrypto-*` bundles the PQClean C reference implementations.

pub mod signatures {
    use pqcrypto_mldsa::mldsa87;
    use pqcrypto_traits::sign::{DetachedSignature as _, PublicKey as _, SecretKey as _};

    pub struct MlDsaKeypair {
        pk: mldsa87::PublicKey,
        sk: mldsa87::SecretKey,
    }

    impl MlDsaKeypair {
        pub fn generate() -> Self {
            let (pk, sk) = mldsa87::keypair();
            Self { pk, sk }
        }

        pub fn public_key_bytes(&self) -> Vec<u8> {
            self.pk.as_bytes().to_vec()
        }

        pub fn secret_key_bytes(&self) -> Vec<u8> {
            self.sk.as_bytes().to_vec()
        }

        pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
            mldsa87::detached_sign(msg, &self.sk).as_bytes().to_vec()
        }

        /// Standalone verifier — decodes pk + sig from bytes, verifies signature.
        pub fn verify(pk_bytes: &[u8], msg: &[u8], sig_bytes: &[u8]) -> bool {
            let pk = match mldsa87::PublicKey::from_bytes(pk_bytes) {
                Ok(k) => k, Err(_) => return false,
            };
            let sig = match mldsa87::DetachedSignature::from_bytes(sig_bytes) {
                Ok(s) => s, Err(_) => return false,
            };
            mldsa87::verify_detached_signature(&sig, msg, &pk).is_ok()
        }
    }
}

pub mod kem {
    use pqcrypto_mlkem::mlkem1024;
    use pqcrypto_traits::kem::{Ciphertext as _, PublicKey as _, SharedSecret as _};

    pub struct MlKemKeypair {
        pk: mlkem1024::PublicKey,
        sk: mlkem1024::SecretKey,
    }

    impl MlKemKeypair {
        pub fn generate() -> Self {
            let (pk, sk) = mlkem1024::keypair();
            Self { pk, sk }
        }

        pub fn public_key_bytes(&self) -> Vec<u8> {
            self.pk.as_bytes().to_vec()
        }

        pub fn encapsulate(&self) -> (Vec<u8>, [u8; 32]) {
            let (ss, ct) = mlkem1024::encapsulate(&self.pk);
            let ss_bytes = ss.as_bytes();
            let mut ss32 = [0u8; 32];
            let n = ss_bytes.len().min(32);
            ss32[..n].copy_from_slice(&ss_bytes[..n]);
            (ct.as_bytes().to_vec(), ss32)
        }

        pub fn decapsulate(&self, ct_bytes: &[u8]) -> Option<[u8; 32]> {
            let ct = mlkem1024::Ciphertext::from_bytes(ct_bytes).ok()?;
            let ss = mlkem1024::decapsulate(&ct, &self.sk);
            let ss_bytes = ss.as_bytes();
            let mut ss32 = [0u8; 32];
            let n = ss_bytes.len().min(32);
            ss32[..n].copy_from_slice(&ss_bytes[..n]);
            Some(ss32)
        }
    }
}

pub use signatures::MlDsaKeypair;
pub use kem::MlKemKeypair;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mldsa_sign_verify_roundtrip() {
        let kp = MlDsaKeypair::generate();
        let msg = b"donkai attestation payload";
        let sig = kp.sign(msg);
        assert!(MlDsaKeypair::verify(&kp.public_key_bytes(), msg, &sig));
    }

    #[test]
    fn mldsa_rejects_tampered_message() {
        let kp = MlDsaKeypair::generate();
        let sig = kp.sign(b"original");
        assert!(!MlDsaKeypair::verify(&kp.public_key_bytes(), b"tampered", &sig));
    }

    #[test]
    fn mldsa_rejects_tampered_signature() {
        let kp = MlDsaKeypair::generate();
        let mut sig = kp.sign(b"payload");
        sig[0] ^= 0xFF;
        assert!(!MlDsaKeypair::verify(&kp.public_key_bytes(), b"payload", &sig));
    }

    #[test]
    fn mldsa_rejects_wrong_pubkey() {
        let kp1 = MlDsaKeypair::generate();
        let kp2 = MlDsaKeypair::generate();
        let sig = kp1.sign(b"payload");
        assert!(!MlDsaKeypair::verify(&kp2.public_key_bytes(), b"payload", &sig));
    }

    #[test]
    fn mlkem_encap_decap_roundtrip() {
        let kp = MlKemKeypair::generate();
        let (ct, ss_sender) = kp.encapsulate();
        let ss_receiver = kp.decapsulate(&ct).expect("decap ok");
        assert_eq!(ss_sender, ss_receiver);
    }
}
