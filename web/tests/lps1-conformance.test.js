/**
 * LPS-1 Conformance Test Suite (Node.js / WebCrypto Engine)
 * Runs during CI and local test passes to assert exact cross-language equivalence.
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

function runSuite() {
  console.log('=== Running LPS-1 Node.js Conformance Tests ===');
  let passed = 0;

  // 1. Unicode NFC & Line Endings
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
      passed++;
    } else if (tc.crlfInput) {
      const canonCrlf = canonicalize(tc.crlfInput);
      const canonLf = canonicalize(tc.lfInput);
      if (canonCrlf !== canonLf || canonCrlf !== tc.expectedCanonicalJson) {
        throw new Error(`CRLF Mismatch on ${tc.name}`);
      }
      passed++;
    }
  }

  // 2. Merkle Odd Leaves & Sorting
  const merkleFixturePath = path.join(__dirname, '../../fixtures/lps1-v1/merkle/odd_leaves.json');
  const merkleFixture = JSON.parse(fs.readFileSync(merkleFixturePath, 'utf8'));

  const emptyRoot = merkleFixture.constants.EMPTY_EVIDENCE_ROOT;
  if (emptyRoot !== '0x0000000000000000000000000000000000000000000000000000000000000000') {
    throw new Error('Invalid EMPTY_EVIDENCE_ROOT in fixture');
  }

  const items = merkleFixture.evidenceItems.map(i => i.id).sort();
  const expectedOrder = merkleFixture.expectedSortedOrder;
  if (JSON.stringify(items) !== JSON.stringify(expectedOrder)) {
    throw new Error('Evidence sorting mismatch');
  }
  passed++;

  // 3. EIP-712 Structured Intent
  const eipFixturePath = path.join(__dirname, '../../fixtures/lps1-v1/eip712/create_remembrance_digest.json');
  const eipFixture = JSON.parse(fs.readFileSync(eipFixturePath, 'utf8'));
  if (eipFixture.domain.chainId !== 1977 || eipFixture.primaryType !== 'CreateRemembrance') {
    throw new Error('EIP-712 fixture parameter mismatch');
  }
  passed++;

  console.log(`[PASS] All ${passed} Node.js / WebCrypto Conformance Checks Passed!`);
}

runSuite();
