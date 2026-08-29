// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title DonkaiPrivacyRegistry
 * @author DONK AI Core Protocol Engineering
 * @notice Manages access policy hashes, key distribution epoch roots, and access revocation tombstones.
 */
contract DonkaiPrivacyRegistry {
    struct AccessPolicy {
        bytes32 policyHash;
        bytes32 keyEpochRoot;
        uint64 setAt;
        bool isRevoked;
    }

    mapping(uint256 => AccessPolicy) public memoryPolicies;

    event PolicyRegistered(uint256 indexed memoryId, bytes32 policyHash, bytes32 keyEpochRoot, uint64 setAt);
    event PolicyRevoked(uint256 indexed memoryId, bytes32 tombstoneHash, uint64 revokedAt);

    function setAccessPolicy(uint256 memoryId, bytes32 policyHash, bytes32 keyEpochRoot) external {
        memoryPolicies[memoryId] = AccessPolicy({
            policyHash: policyHash,
            keyEpochRoot: keyEpochRoot,
            setAt: uint64(block.timestamp),
            isRevoked: false
        });
        emit PolicyRegistered(memoryId, policyHash, keyEpochRoot, uint64(block.timestamp));
    }

    function revokeAccessPolicy(uint256 memoryId, bytes32 tombstoneHash) external {
        AccessPolicy storage p = memoryPolicies[memoryId];
        p.isRevoked = true;
        emit PolicyRevoked(memoryId, tombstoneHash, uint64(block.timestamp));
    }
}
