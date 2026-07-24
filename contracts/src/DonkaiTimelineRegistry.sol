// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.24;

import {ERC721}              from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {ERC721Enumerable}    from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import {ERC721URIStorage}    from "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import {AccessControl}       from "@openzeppelin/contracts/access/AccessControl.sol";
import {Pausable}            from "@openzeppelin/contracts/utils/Pausable.sol";
import {IIdentityRegistry}   from "./interfaces/external/IIdentityRegistry.sol";

import {DonkaiErrors}        from "./libraries/DonkaiErrors.sol";

/// @title  DonkaiTimelineRegistry
/// @author Donkai Core Engineering
/// @notice Community-minted historical "relic" NFTs, anchored by a 32-byte LPS-1
///         Merkle root (produced off-chain by the `donkai-lps1` Rust crate) and
///         an IPFS CID (produced by `donkai-ipfs`). Optional ERC-3643 identity
///         gate on mint via T-REX `IIdentityRegistry.isVerified`.
///
///         Policy: **one Merkle root, one relic.** Duplicate roots are rejected —
///         the same document is a canonical artifact regardless of who mints it.
contract DonkaiTimelineRegistry is ERC721Enumerable, ERC721URIStorage, AccessControl, Pausable {
    // ------------------------------------------------------------------ roles
    bytes32 public constant PASQUALLY_ORACLE_ROLE = keccak256("PASQUALLY_ORACLE_ROLE");
    bytes32 public constant PAUSER_ROLE           = keccak256("PAUSER_ROLE");
    bytes32 public constant REGISTRY_ADMIN_ROLE   = keccak256("REGISTRY_ADMIN_ROLE");

    // ------------------------------------------------------------ constants
    uint256 public constant MAX_TITLE_LEN    = 128;
    uint256 public constant MAX_IPFS_CID_LEN = 128;
    uint256 public constant MIN_IPFS_CID_LEN = 5;      // shortest plausible bafk... / Qm... prefix
    uint8   public constant MAX_STUBBORNNESS = 100;

    /// @notice Historical zone taxonomy — mirrors the 5-era arc in the whitepaper.
    enum Era { PreDigital, PhysicalArcade, PaperTicket, DigitalMicrotrans, AgenticYield }

    struct Relic {
        string  title;
        string  ipfsCID;
        bytes32 lps1MerkleRoot;
        Era     era;
        uint8   stubbornnessScore;   // 0..100, oracle-attested (0 at mint)
        address minter;
        uint64  mintedAt;
    }

    // -------------------------------------------------------------- storage
    IIdentityRegistry public identityRegistry;
    uint256 private _nextId = 1;
    mapping(uint256 => Relic)  private _relics;
    mapping(bytes32 => uint256) private _tokenIdByRoot;

    // --------------------------------------------------------------- events
    event RelicMinted(
        uint256 indexed tokenId,
        address indexed minter,
        bytes32 indexed lps1MerkleRoot,
        Era era,
        string ipfsCID,
        string title
    );
    event StubbornnessAttested(uint256 indexed tokenId, uint8 score, address indexed oracle);
    event IdentityRegistryUpdated(address indexed previous, address indexed current);

    // ---------------------------------------------------------- constructor
    constructor(address admin, address initialOracle, address initialIdentityRegistry)
        ERC721("Donkai Relic", "DKRELIC")
    {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(REGISTRY_ADMIN_ROLE, admin);
        _grantRole(PAUSER_ROLE,         admin);
        if (initialOracle != address(0)) _grantRole(PASQUALLY_ORACLE_ROLE, initialOracle);
        identityRegistry = IIdentityRegistry(initialIdentityRegistry);
        emit IdentityRegistryUpdated(address(0), initialIdentityRegistry);
    }

    // ------------------------------------------------------------- mint
    /// @notice Mint a new Relic NFT.
    /// @dev    Reverts on zero root, duplicate root, empty/overlong strings,
    ///         or failed identity gate (when configured).
    function mintRelic(
        string calldata title,
        string calldata ipfsCID,
        bytes32 lps1MerkleRoot,
        Era era
    ) external whenNotPaused returns (uint256 tokenId) {
        if (lps1MerkleRoot == bytes32(0)) revert DonkaiErrors.ZeroMerkleRoot();
        uint256 existing = _tokenIdByRoot[lps1MerkleRoot];
        if (existing != 0) revert DonkaiErrors.DuplicateMerkleRoot(lps1MerkleRoot, existing);

        uint256 titleLen = bytes(title).length;
        if (titleLen == 0) revert DonkaiErrors.EmptyTitle();
        if (titleLen > MAX_TITLE_LEN) revert DonkaiErrors.TitleTooLong(titleLen);

        uint256 cidLen = bytes(ipfsCID).length;
        if (cidLen < MIN_IPFS_CID_LEN) revert DonkaiErrors.EmptyIpfsCid();
        if (cidLen > MAX_IPFS_CID_LEN) revert DonkaiErrors.IpfsCidTooLong(cidLen);

        if (address(identityRegistry) != address(0)) {
            if (!identityRegistry.isVerified(msg.sender)) {
                revert DonkaiErrors.IdentityNotVerified(msg.sender);
            }
        }

        tokenId = _nextId++;
        _relics[tokenId] = Relic({
            title:             title,
            ipfsCID:           ipfsCID,
            lps1MerkleRoot:    lps1MerkleRoot,
            era:               era,
            stubbornnessScore: 0,
            minter:            msg.sender,
            mintedAt:          uint64(block.timestamp)
        });
        _tokenIdByRoot[lps1MerkleRoot] = tokenId;

        _safeMint(msg.sender, tokenId);
        _setTokenURI(tokenId, string.concat("ipfs://", ipfsCID));

        emit RelicMinted(tokenId, msg.sender, lps1MerkleRoot, era, ipfsCID, title);
    }

    // ------------------------------------------------------- oracle attest
    /// @notice Oracle attests a stubbornness score in [0, 100].
    function attestStubbornness(uint256 tokenId, uint8 score) external onlyRole(PASQUALLY_ORACLE_ROLE) {
        if (score > MAX_STUBBORNNESS) revert DonkaiErrors.ScoreOutOfRange(score);
        _requireOwned(tokenId);
        _relics[tokenId].stubbornnessScore = score;
        emit StubbornnessAttested(tokenId, score, msg.sender);
    }

    // ----------------------------------------------------------- admin
    function setIdentityRegistry(address newRegistry) external onlyRole(REGISTRY_ADMIN_ROLE) {
        address prev = address(identityRegistry);
        identityRegistry = IIdentityRegistry(newRegistry);
        emit IdentityRegistryUpdated(prev, newRegistry);
    }

    function pause()   external onlyRole(PAUSER_ROLE) { _pause();   }
    function unpause() external onlyRole(PAUSER_ROLE) { _unpause(); }

    // ------------------------------------------------------------ views
    function getRelic(uint256 tokenId) external view returns (Relic memory) {
        _requireOwned(tokenId);
        return _relics[tokenId];
    }

    function tokenIdByRoot(bytes32 lps1MerkleRoot) external view returns (uint256) {
        return _tokenIdByRoot[lps1MerkleRoot];
    }

    // ---------------------------------------- inheritance overrides (OZ v5)
    function _update(address to, uint256 tokenId, address auth)
        internal
        override(ERC721, ERC721Enumerable)
        returns (address)
    {
        return super._update(to, tokenId, auth);
    }

    function _increaseBalance(address account, uint128 value)
        internal
        override(ERC721, ERC721Enumerable)
    {
        super._increaseBalance(account, value);
    }

    function tokenURI(uint256 tokenId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (string memory)
    {
        return super.tokenURI(tokenId);
    }

    function supportsInterface(bytes4 interfaceId)
        public
        view
        override(ERC721Enumerable, ERC721URIStorage, AccessControl)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}
