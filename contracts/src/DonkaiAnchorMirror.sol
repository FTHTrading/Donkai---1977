// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.24;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

/// @title  DonkaiAnchorMirror
/// @author Donkai Core Engineering
/// @notice Minimal Polygon-side (Chain ID 137) commitment log for LPS-1 Merkle
///         roots originating on Donkai devnet (Chain ID 1977). Not the full
///         registry — one row per root, cross-chain provenance only.
contract DonkaiAnchorMirror is AccessControl {
    bytes32 public constant ANCHOR_SUBMITTER_ROLE = keccak256("ANCHOR_SUBMITTER_ROLE");

    struct AnchorCommit {
        bytes32 lps1MerkleRoot;
        uint256 sourceChainId;
        uint256 sourceTokenId;
        uint64  timestamp;
        address submitter;
    }

    uint256 public nextAnchorId = 1;
    mapping(uint256 => AnchorCommit) private _anchors;
    mapping(bytes32 => uint256)      private _anchorIdByRoot;

    event AnchorCommitted(
        uint256 indexed anchorId,
        bytes32 indexed lps1MerkleRoot,
        uint256 sourceChainId,
        uint256 sourceTokenId,
        address indexed submitter
    );

    error ZeroAnchorRoot();
    error DuplicateAnchorRoot(bytes32 root, uint256 existingAnchorId);
    error AnchorNotFound(uint256 anchorId);

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(ANCHOR_SUBMITTER_ROLE, admin);
    }

    /// @notice Commit an LPS-1 Merkle root originating from `sourceChainId`.
    function commitRoot(bytes32 lps1MerkleRoot, uint256 sourceChainId, uint256 sourceTokenId)
        external
        onlyRole(ANCHOR_SUBMITTER_ROLE)
        returns (uint256 anchorId)
    {
        if (lps1MerkleRoot == bytes32(0)) revert ZeroAnchorRoot();
        uint256 existing = _anchorIdByRoot[lps1MerkleRoot];
        if (existing != 0) revert DuplicateAnchorRoot(lps1MerkleRoot, existing);

        anchorId = nextAnchorId++;
        _anchors[anchorId] = AnchorCommit({
            lps1MerkleRoot: lps1MerkleRoot,
            sourceChainId:  sourceChainId,
            sourceTokenId:  sourceTokenId,
            timestamp:      uint64(block.timestamp),
            submitter:      msg.sender
        });
        _anchorIdByRoot[lps1MerkleRoot] = anchorId;

        emit AnchorCommitted(anchorId, lps1MerkleRoot, sourceChainId, sourceTokenId, msg.sender);
    }

    function getAnchor(uint256 anchorId) external view returns (AnchorCommit memory) {
        AnchorCommit memory a = _anchors[anchorId];
        if (a.timestamp == 0) revert AnchorNotFound(anchorId);
        return a;
    }

    function anchorIdByRoot(bytes32 lps1MerkleRoot) external view returns (uint256) {
        return _anchorIdByRoot[lps1MerkleRoot];
    }
}
