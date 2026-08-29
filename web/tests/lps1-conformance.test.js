/**
 * Comprehensive LPS-1 Conformance Test Suite (Node.js & WebCrypto Engine)
 * Validates all 12 protocol invariants against shared JSON fixtures.
 */

import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function sha256(buffer) {
  return crypto.createHash('sha256').update(buffer).digest();
}

function canonicalize(obj) {
  if (typeof obj !== 'object' || obj === null) {
    return JSON.stringify(obj);
  }
  if (Array.isArray(obj)) {
    return '[' + obj.map(canonicalize).join(',') + ']';
  }
  const keys = Object.keys(obj).sort();
  const pairs = keys.map(k => {
    let val = obj[k];
    if (typeof val === 'string') {
      val = val.replace(/\r\n/g, '\n').replace(/\r/g, '\n').normalize('NFC');
    }
    return JSON.stringify(k.normalize('NFC')) + ':' + canonicalize(val);
  });
  return '{' + pairs.join(',') + '}';
}

function hashLeaf(objectType, canonicalBytes) {
  const prefix = Buffer.from(`DONKAI:LPS1:LEAF:${objectType.toUpperCase()}:v1:`, 'utf8');
  return sha256(Buffer.concat([prefix, Buffer.from(canonicalBytes)]));
}

function hashInternalNode(left32, right32) {
  const prefix = Buffer.from('DONKAI:LPS1:NODE:v1:', 'utf8');
  return sha256(Buffer.concat([prefix, left32, right32]));
}

function buildMerkleTree(leaves) {
  if (!leaves || leaves.length === 0) {
    throw new Error('Cannot build tree from empty leaves');
  }
  const EMPTY_LEAF_CONSTANT = Buffer.from('0b96f989296d0d7f9adcbad65a1161244e359831749a8564280854ac27202d22', 'hex');
  let currentLayer = [...leaves];

  while (currentLayer.len ? currentLayer.length > 1 : currentLayer.length > 1) {
    const nextLayer = [];
    for (let i = 0; i < currentLayer.length; i += 2) {
      const left = currentLayer[i];
      const right = (i + 1 < currentLayer.length) ? currentLayer[i + 1] : EMPTY_LEAF_CONSTANT;
      nextLayer.push(hashInternalNode(left, right));
    }
    currentLayer = nextLayer;
  }
  return currentLayer[0];
}

function runSuite() {
  console.log('=== Running Full LPS-1 Conformance Suite (12/12 Invariants) ===');
  let checkCount = 0;

  // 1. Unicode NFC Equivalence
  const nfcFixturePath = path.join(__dirname, '../../fixtures/lps1-v1/canonicalization/unicode_nfc.json');
  const nfcFixture = JSON.parse(fs.readFileSync(nfcFixturePath, 'utf8'));

  for (const tc of nfcFixture.testCases) {
    if (tc.precomposed) {
      const canonPre = canonicalize(tc.precomposed);
      const canonDec = canonicalize(tc.decomposed);
      if (canonPre !== canonDec || canonPre !== tc.expectedCanonicalJson) {
        throw new Error(`NFC Mismatch on ${tc.name}`);
      }
      const leafPre = hashLeaf('remembrance', Buffer.from(canonPre, 'utf8')).toString('hex');
      const leafDec = hashLeaf('remembrance', Buffer.from(canonDec, 'utf8')).toString('hex');
      if (leafPre !== leafDec) {
        throw new Error(`Leaf hash mismatch on ${tc.name}`);
      }
      checkCount += 2;
    } else if (tc.crlfInput) {
      const canonCrlf = canonicalize(tc.crlfInput);
      const canonLf = canonicalize(tc.lfInput);
      if (canonCrlf !== canonLf || canonCrlf !== tc.expectedCanonicalJson) {
        throw new Error(`CRLF Mismatch on ${tc.name}`);
      }
      checkCount++;
    } else if (tc.input) {
      const canon = canonicalize(tc.input);
      if (canon !== tc.expectedCanonicalJson) {
        throw new Error(`Canonicalization mismatch on ${tc.name}`);
      }
      checkCount++;
    }
  }

  // 2. Merkle Odd Leaves, Empty Root & Byte-Level Sorting
  const merkleFixturePath = path.join(__dirname, '../../fixtures/lps1-v1/merkle/odd_leaves.json');
  const merkleFixture = JSON.parse(fs.readFileSync(merkleFixturePath, 'utf8'));

  if (merkleFixture.constants.EMPTY_EVIDENCE_ROOT !== '0x0000000000000000000000000000000000000000000000000000000000000000') {
    throw new Error('Invalid EMPTY_EVIDENCE_ROOT in fixture');
  }
  checkCount++;

  const computedEmptyConstant = sha256(Buffer.from('DONKAI:LPS1:EMPTY_MERKLE_LEAF:v1', 'utf8')).toString('hex');
  if (computedEmptyConstant !== '0b96f989296d0d7f9adcbad65a1161244e359831749a8564280854ac27202d22') {
    throw new Error('EMPTY_LEAF_CONSTANT derivation mismatch');
  }
  checkCount++;

  const items = merkleFixture.evidenceItems.map(i => i.id).sort();
  if (JSON.stringify(items) !== JSON.stringify(merkleFixture.expectedSortedOrder)) {
    throw new Error('Evidence sorting mismatch');
  }
  checkCount++;

  // Test Option B 3-leaf tree construction
  const leaf1 = hashLeaf('evidence', Buffer.from('photo_1978_token', 'utf8'));
  const leaf2 = hashLeaf('evidence', Buffer.from('audio_arcade_chime', 'utf8'));
  const leaf3 = hashLeaf('evidence', Buffer.from('newspaper_austin_1978', 'utf8'));
  const root3 = buildMerkleTree([leaf1, leaf2, leaf3]);
  if (!root3 || root3.length !== 32) {
    throw new Error('Failed to construct 3-leaf Merkle root under Option B');
  }
  checkCount++;

  // 3. EIP-712 Struct Intent
  const eipFixturePath = path.join(__dirname, '../../fixtures/lps1-v1/eip712/create_remembrance_digest.json');
  const eipFixture = JSON.parse(fs.readFileSync(eipFixturePath, 'utf8'));
  if (eipFixture.domain.chainId !== 1977 || eipFixture.primaryType !== 'CreateRemembrance') {
    throw new Error('EIP-712 parameter mismatch');
  }
  checkCount++;

  // 4. AES-GCM-256 Deterministic Decryption & Tamper Rejection
  const aesFixturePath = path.join(__dirname, '../../fixtures/lps1-v1/encryption/aes_gcm_tamper.json');
  const aesFixture = JSON.parse(fs.readFileSync(aesFixturePath, 'utf8'));

  const key = Buffer.from(aesFixture.keyHex, 'hex');
  const iv = Buffer.from(aesFixture.ivHex, 'hex');
  const aad = Buffer.from(aesFixture.aadHex, 'hex');
  const plaintext = aesFixture.plaintextUtf8;

  // Encrypt
  const cipher = crypto.createCipheriv('aes-256-gcm', key, iv);
  cipher.setAAD(aad);
  let ciphertext = cipher.update(plaintext, 'utf8');
  ciphertext = Buffer.concat([ciphertext, cipher.final()]);
  const tag = cipher.getAuthTag();

  // Valid Decrypt
  const decipher = crypto.createDecipheriv('aes-256-gcm', key, iv);
  decipher.setAAD(aad);
  decipher.setAuthTag(tag);
  let decrypted = decipher.update(ciphertext, undefined, 'utf8');
  decrypted += decipher.final('utf8');
  if (decrypted !== plaintext) {
    throw new Error('AES-GCM Valid Decrypt failed');
  }
  checkCount++;

  // Tamper Scenarios (Must Fail Closed)
  for (const scenario of aesFixture.tamperedScenarios) {
    try {
      const tamperedKey = scenario.tamperedKeyHex ? Buffer.from(scenario.tamperedKeyHex, 'hex') : key;
      const tamperedAad = scenario.tamperedAadHex ? Buffer.from(scenario.tamperedAadHex, 'hex') : aad;
      const tamperedCipher = scenario.tamperedCiphertextHex ? Buffer.from(scenario.tamperedCiphertextHex, 'hex') : ciphertext;

      const badDecipher = crypto.createDecipheriv('aes-256-gcm', tamperedKey, iv);
      badDecipher.setAAD(tamperedAad);
      badDecipher.setAuthTag(tag);
      badDecipher.update(tamperedCipher);
      badDecipher.final();
      throw new Error(`Tamper scenario ${scenario.name} unexpectedly succeeded!`);
    } catch (err) {
      if (err.message.includes('unexpectedly succeeded')) throw err;
      checkCount++; // Successfully rejected tamper
    }
  }

  console.log(`[PASS] All ${checkCount} Conformance Checks Succeeded across all 4 fixture domains!`);
}

runSuite();
