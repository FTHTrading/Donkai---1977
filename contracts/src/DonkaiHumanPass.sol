// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title DonkaiHumanPass
 * @author DONK AI Core Protocol Engineering
 * @notice Non-transferable identity credential for baseline participation, sybil resistance,
 * and role-based permissions in the Human Remembrance Protocol.
 * Non-financial: cannot be transferred, bought, or sold.
 */
contract DonkaiHumanPass {
    enum Role {
        Author,
        Corroborator,
        EvidenceCustodian,
        Reviewer,
        InstitutionalVerifier,
        Translator,
        AppealsPanel,
        ProtocolSteward
    }

    struct Pass {
        address holder;
        bytes32 identityCommitment;
        uint64 issuedAt;
        bool isActive;
        mapping(Role => bool) roles;
    }

    address public steward;
    uint256 private _nextPassId = 1;
    mapping(uint256 => Pass) private _passes;
    mapping(address => uint256) public holderToPassId;

    event HumanPassIssued(uint256 indexed passId, address indexed holder, bytes32 identityCommitment, uint64 issuedAt);
    event RoleAssigned(uint256 indexed passId, Role role);
    event RoleRevoked(uint256 indexed passId, Role role);
    event HumanPassRevoked(uint256 indexed passId, address indexed holder);

    error OnlySteward();
    error PassAlreadyExists(address holder);
    error PassNotFound(uint256 passId);
    error NonTransferable();

    modifier onlySteward() {
        if (msg.sender != steward) revert OnlySteward();
        _;
    }

    constructor() {
        steward = msg.sender;
    }

    function issuePass(address holder, bytes32 identityCommitment) external onlySteward returns (uint256 passId) {
        if (holderToPassId[holder] != 0) revert PassAlreadyExists(holder);

        passId = _nextPassId++;
        Pass storage p = _passes[passId];
        p.holder = holder;
        p.identityCommitment = identityCommitment;
        p.issuedAt = uint64(block.timestamp);
        p.isActive = true;
        p.roles[Role.Author] = true;
        p.roles[Role.Corroborator] = true;

        holderToPassId[holder] = passId;

        emit HumanPassIssued(passId, holder, identityCommitment, uint64(block.timestamp));
        emit RoleAssigned(passId, Role.Author);
        emit RoleAssigned(passId, Role.Corroborator);
    }

    function assignRole(uint256 passId, Role role) external onlySteward {
        Pass storage p = _passes[passId];
        if (!p.isActive) revert PassNotFound(passId);
        p.roles[role] = true;
        emit RoleAssigned(passId, role);
    }

    function revokeRole(uint256 passId, Role role) external onlySteward {
        Pass storage p = _passes[passId];
        if (!p.isActive) revert PassNotFound(passId);
        p.roles[role] = false;
        emit RoleRevoked(passId, role);
    }

    function hasRole(address holder, Role role) external view returns (bool) {
        uint256 passId = holderToPassId[holder];
        if (passId == 0) return false;
        Pass storage p = _passes[passId];
        if (!p.isActive) return false;
        return p.roles[role];
    }

    function isHumanVerified(address holder) external view returns (bool) {
        uint256 passId = holderToPassId[holder];
        if (passId == 0) return false;
        return _passes[passId].isActive;
    }
}
