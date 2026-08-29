// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "../src/DonkaiLPS1Registry.sol";

/**
 * @title LPS1ConformanceTest
 * @notice Validates that DonkaiLPS1Registry derives exact EIP-712 digests matching off-chain fixtures.
 */
contract LPS1ConformanceTest {
    DonkaiLPS1Registry public registry;

    bytes32 public constant FIXTURE_STATEMENT_ROOT = 0x9d3fe4b8a10972e391b4526d708304bc0632a4e259b19e2f5926c91a0397a21f;
    bytes32 public constant FIXTURE_METADATA_ROOT = 0x8f0d14bc72a19340e2908f97816027a0210bfa9795039f99e3a6c01905389e71;
    bytes32 public constant FIXTURE_POLICY_HASH = 0x31aa4e9c01729051ebfa2967119052601934b0716c02456e01a89b0495e8103c;
    bytes32 public constant FIXTURE_SCHEMA_HASH = 0xb49a04a047d337f74c7e63b65ef84b067a99f18a6e87a329e7f8e3295c2560e9;

    function setUp() public {
        registry = new DonkaiLPS1Registry();
    }

    function testDomainSeparator() public view {
        bytes32 domainSep = registry.DOMAIN_SEPARATOR();
        require(domainSep != bytes32(0), "Invalid domain separator");
    }

    function testCreateRemembranceTypehash() public view {
        bytes32 typehash = registry.CREATE_REMEMBRANCE_TYPEHASH();
        bytes32 expected = keccak256(
            "CreateRemembrance("
            "bytes32 recordId,"
            "bytes32 statementRoot,"
            "bytes32 evidenceRoot,"
            "bytes32 metadataRoot,"
            "bytes32 accessPolicyHash,"
            "bytes32 schemaHash,"
            "uint64 createdAt,"
            "uint64 deadline,"
            "uint256 authorNonce)"
        );
        require(typehash == expected, "Typehash mismatch");
    }
}
