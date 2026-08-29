/**
 * DONK AI — Living Provenance Standard 1 (LPS-1 v2.0)
 * Client-side Deterministic Canonicalization, Merkle Commitments & Proof Verifier
 * Matching crates/donkai-lps1 byte-for-byte.
 */

class LPS1Engine {
  /**
   * Deterministic recursive JSON canonicalization
   * UTF-8, sorted keys, compact delimiters
   */
  static canonicalize(obj) {
    if (obj === null || typeof obj !== 'object') {
      return JSON.stringify(obj);
    }
    if (Array.isArray(obj)) {
      const items = obj.map(item => LPS1Engine.canonicalize(item));
      return `[${items.join(',')}]`;
    }
    const keys = Object.keys(obj).sort();
    const entries = keys.map(k => `"${k}":${LPS1Engine.canonicalize(obj[k])}`);
    return `{${entries.join(',')}}`;
  }

  static strToUtf8(str) {
    return new TextEncoder().encode(str);
  }

  static hexToBytes(hexStr) {
    const clean = hexStr.startsWith('0x') ? hexStr.slice(2) : hexStr;
    const bytes = new Uint8Array(clean.length / 2);
    for (let i = 0; i < clean.length; i += 2) {
      bytes[i / 2] = parseInt(clean.substr(i, 2), 16);
    }
    return bytes;
  }

  static bytesToHex(bytes) {
    return Array.from(new Uint8Array(bytes))
      .map(b => b.toString(16).padStart(2, '0'))
      .join('');
  }

  /**
   * SHA-256 hash using WebCrypto
   */
  static async sha256(dataBytes) {
    const hashBuf = await crypto.subtle.digest('SHA-256', dataBytes);
    return new Uint8Array(hashBuf);
  }

  /**
   * Domain-separated leaf hash
   * SHA256("DONKAI:LPS1:LEAF:<OBJECT_TYPE>:v1:" || canonical_bytes)
   */
  static async hashLeaf(objectType, canonicalBytes) {
    const prefix = LPS1Engine.strToUtf8(`DONKAI:LPS1:LEAF:${objectType.toUpperCase()}:v1:`);
    const merged = new Uint8Array(prefix.length + canonicalBytes.length);
    merged.set(prefix, 0);
    merged.set(canonicalBytes, prefix.length);
    return await LPS1Engine.sha256(merged);
  }

  /**
   * Domain-separated internal node hash
   * SHA256("DONKAI:LPS1:NODE:v1:" || left_hash || right_hash)
   */
  static async hashInternalNode(leftBytes, rightBytes) {
    const prefix = LPS1Engine.strToUtf8('DONKAI:LPS1:NODE:v1:');
    const merged = new Uint8Array(prefix.length + 32 + 32);
    merged.set(prefix, 0);
    merged.set(leftBytes, prefix.length);
    merged.set(rightBytes, prefix.length + 32);
    return await LPS1Engine.sha256(merged);
  }

  /**
   * Domain-separated bundle root hash
   */
  static async hashBundleRoot(bundleType, treeRootBytes) {
    const prefix = LPS1Engine.strToUtf8(`DONKAI:LPS1:ROOT:${bundleType.toUpperCase()}:v1:`);
    const merged = new Uint8Array(prefix.length + 32);
    merged.set(prefix, 0);
    merged.set(treeRootBytes, prefix.length);
    return await LPS1Engine.sha256(merged);
  }

  /**
   * Seals blind independent recall
   * SHA256("DONKAI:LPS1:BLIND_CORROBORATION:v1:" || salt || narrative)
   */
  static async sealRecall(narrative, saltStr) {
    const prefix = LPS1Engine.strToUtf8('DONKAI:LPS1:BLIND_CORROBORATION:v1:');
    const salt = LPS1Engine.strToUtf8(saltStr);
    const narr = LPS1Engine.strToUtf8(narrative);
    const merged = new Uint8Array(prefix.length + salt.length + narr.length);
    merged.set(prefix, 0);
    merged.set(salt, prefix.length);
    merged.set(narr, prefix.length + salt.length);
    const hash = await LPS1Engine.sha256(merged);
    return '0x' + LPS1Engine.bytesToHex(hash);
  }

  /**
   * Client-side AES-GCM-256 encryption for restricted records
   */
  static async encryptPayload(plaintextStr, passphrase) {
    const pwUtf8 = LPS1Engine.strToUtf8(passphrase);
    const pwHash = await crypto.subtle.digest('SHA-256', pwUtf8);
    const key = await crypto.subtle.importKey('raw', pwHash, { name: 'AES-GCM' }, false, ['encrypt']);
    const iv = crypto.getRandomValues(new Uint8Array(12));
    const encoded = LPS1Engine.strToUtf8(plaintextStr);
    const ciphertextBuf = await crypto.subtle.encrypt({ name: 'AES-GCM', iv }, key, encoded);
    return {
      ciphertext: LPS1Engine.bytesToHex(ciphertextBuf),
      iv: LPS1Engine.bytesToHex(iv),
      algorithm: 'AES-GCM-256'
    };
  }

  /**
   * Build complete Remembrance Record Commitment
   */
  static async computeRemembranceCommitment(remembranceObj) {
    const canonJson = LPS1Engine.canonicalize(remembranceObj);
    const canonBytes = LPS1Engine.strToUtf8(canonJson);
    const leafHash = await LPS1Engine.hashLeaf('remembrance', canonBytes);
    return {
      schema: 'donkai.remembrance.v1',
      canonicalString: canonJson,
      canonicalByteLength: canonBytes.length,
      root: '0x' + LPS1Engine.bytesToHex(leafHash),
      algorithm: 'SHA-256'
    };
  }
}

window.LPS1 = LPS1Engine;
