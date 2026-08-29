// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title DonkaiBlindCorroboration
 * @author DONK AI Core Protocol Engineering
 * @notice Enforces the blind-first commit-reveal protocol for independent human recollections.
 * Prevents anchoring bias and bandwagon effects by ensuring contributors seal their recall
 * against neutral discovery metadata prior to viewing full submitted testimony.
 */
contract DonkaiBlindCorroboration {
    struct SealedCorroboration {
        uint256 memoryId;
        address contributor;
        bytes32 sealedRecallRoot;
        bytes32 discoveryContextHash;
        uint64 committedAt;
        uint64 revealedAt;
        bool isRevealed;
    }

    uint256 private _nextCorroborationId = 1;
    mapping(uint256 => SealedCorroboration) public corroborations;
    mapping(uint256 => uint256[]) public memoryCorroborations;
    mapping(uint256 => uint256) public revealedCount;

    event RecallCommitted(
        uint256 indexed corroborationId,
        uint256 indexed memoryId,
        address indexed contributor,
        bytes32 sealedRecallRoot,
        bytes32 discoveryContextHash,
        uint64 committedAt
    );

    event RecallRevealed(
        uint256 indexed corroborationId,
        uint256 indexed memoryId,
        address indexed contributor,
        bytes32 revealedContentHash,
        uint64 revealedAt
    );

    error InvalidSealedRoot();
    error CorroborationNotFound(uint256 corroborationId);
    error UnauthorizedReveal(address caller, address contributor);
    error AlreadyRevealed(uint256 corroborationId);
    error InvalidRevealProof();

    function commitSealedRecall(
        uint256 memoryId,
        bytes32 sealedRecallRoot,
        bytes32 discoveryContextHash
    ) external returns (uint256 corroborationId) {
        if (sealedRecallRoot == bytes32(0)) revert InvalidSealedRoot();

        corroborationId = _nextCorroborationId++;
        corroborations[corroborationId] = SealedCorroboration({
            memoryId: memoryId,
            contributor: msg.sender,
            sealedRecallRoot: sealedRecallRoot,
            discoveryContextHash: discoveryContextHash,
            committedAt: uint64(block.timestamp),
            revealedAt: 0,
            isRevealed: false
        });

        memoryCorroborations[memoryId].push(corroborationId);

        emit RecallCommitted(
            corroborationId,
            memoryId,
            msg.sender,
            sealedRecallRoot,
            discoveryContextHash,
            uint64(block.timestamp)
        );
    }

    function revealRecall(
        uint256 corroborationId,
        bytes calldata narrative,
        bytes32 salt
    ) external {
        SealedCorroboration storage c = corroborations[corroborationId];
        if (c.committedAt == 0) revert CorroborationNotFound(corroborationId);
        if (c.contributor != msg.sender) revert UnauthorizedReveal(msg.sender, c.contributor);
        if (c.isRevealed) revert AlreadyRevealed(corroborationId);

        // Verify SHA256("DONKAI:LPS1:BLIND_CORROBORATION:v1:" || salt || narrative)
        bytes32 computed = sha256(
            abi.encodePacked("DONKAI:LPS1:BLIND_CORROBORATION:v1:", salt, narrative)
        );
        if (computed != c.sealedRecallRoot) revert InvalidRevealProof();

        c.isRevealed = true;
        c.revealedAt = uint64(block.timestamp);
        revealedCount[c.memoryId]++;

        bytes32 contentHash = sha256(narrative);

        emit RecallRevealed(
            corroborationId,
            c.memoryId,
            msg.sender,
            contentHash,
            uint64(block.timestamp)
        );
    }

    function getCorroborationIds(uint256 memoryId) external view returns (uint256[] memory) {
        return memoryCorroborations[memoryId];
    }
}
