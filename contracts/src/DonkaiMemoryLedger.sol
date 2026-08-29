// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title DonkaiMemoryLedger
 * @author DONK AI Core Protocol Engineering
 * @notice Living Provenance Standard 1 (LPS-1) Canonical Memory Ledger.
 * Anchors cryptographic commitments, submission provenance, parent-version graphs,
 * and lifecycle status for human remembrances.
 *
 * Epistemic guarantee:
 * An on-chain commitment proves that a specified canonical record was committed by a
 * signing identity no later than its anchored timestamp. It does NOT establish that the
 * recollection is historically accurate, complete, or exclusive.
 */
contract DonkaiMemoryLedger {
    enum RecordStatus {
        Active,
        Amended,
        Retracted,
        Restricted
    }

    enum VisibilityMode {
        Private,
        ReviewerOnly,
        TrustedCircle,
        DelayedPublic,
        Public,
        AggregateOnly
    }

    enum AmendmentRelation {
        Clarification,
        Correction,
        Redaction,
        Retraction,
        EvidenceSupplement
    }

    struct MemoryCommitment {
        address signer;
        bytes32 statementRoot;
        bytes32 contextRoot;
        bytes32 consentRoot;
        bytes32 evidenceBundleRoot;
        bytes32 parentVersionRoot;
        bytes32 methodologyVersion;
        uint64 committedAt;
        VisibilityMode visibility;
        RecordStatus status;
    }

    uint256 private _nextMemoryId = 1;
    mapping(uint256 => MemoryCommitment) public records;
    mapping(bytes32 => uint256) public rootToMemoryId;
    mapping(uint256 => uint256[]) public versionHistory;

    event MemoryRecordCommitted(
        uint256 indexed memoryId,
        address indexed signer,
        bytes32 indexed statementRoot,
        bytes32 contextRoot,
        bytes32 consentRoot,
        bytes32 evidenceBundleRoot,
        bytes32 parentVersionRoot,
        uint64 committedAt,
        VisibilityMode visibility
    );

    event MemoryRecordAmended(
        uint256 indexed originalMemoryId,
        uint256 indexed newMemoryId,
        address indexed signer,
        bytes32 newStatementRoot,
        AmendmentRelation relation,
        uint64 amendedAt
    );

    event MemoryRecordRetracted(
        uint256 indexed memoryId,
        address indexed signer,
        bytes32 retractionRationaleRoot,
        uint64 retractedAt
    );

    event MemoryRecordStatusUpdated(
        uint256 indexed memoryId,
        RecordStatus newStatus,
        uint64 updatedAt
    );

    error RecordNotFound(uint256 memoryId);
    error UnauthorizedSigner(address caller, address authorized);
    error RootAlreadyCommitted(bytes32 root);
    error InvalidRoot();

    function commitRecord(
        bytes32 statementRoot,
        bytes32 contextRoot,
        bytes32 consentRoot,
        bytes32 evidenceBundleRoot,
        bytes32 parentVersionRoot,
        bytes32 methodologyVersion,
        VisibilityMode visibility
    ) external returns (uint256 memoryId) {
        if (statementRoot == bytes32(0)) revert InvalidRoot();
        if (rootToMemoryId[statementRoot] != 0) revert RootAlreadyCommitted(statementRoot);

        memoryId = _nextMemoryId++;
        records[memoryId] = MemoryCommitment({
            signer: msg.sender,
            statementRoot: statementRoot,
            contextRoot: contextRoot,
            consentRoot: consentRoot,
            evidenceBundleRoot: evidenceBundleRoot,
            parentVersionRoot: parentVersionRoot,
            methodologyVersion: methodologyVersion,
            committedAt: uint64(block.timestamp),
            visibility: visibility,
            status: RecordStatus.Active
        });

        rootToMemoryId[statementRoot] = memoryId;

        emit MemoryRecordCommitted(
            memoryId,
            msg.sender,
            statementRoot,
            contextRoot,
            consentRoot,
            evidenceBundleRoot,
            parentVersionRoot,
            uint64(block.timestamp),
            visibility
        );
    }

    function amendRecord(
        uint256 originalMemoryId,
        bytes32 newStatementRoot,
        bytes32 newContextRoot,
        bytes32 newConsentRoot,
        bytes32 newEvidenceRoot,
        bytes32 methodologyVersion,
        VisibilityMode visibility,
        AmendmentRelation relation
    ) external returns (uint256 newMemoryId) {
        MemoryCommitment storage original = records[originalMemoryId];
        if (original.signer == address(0)) revert RecordNotFound(originalMemoryId);
        if (original.signer != msg.sender) revert UnauthorizedSigner(msg.sender, original.signer);
        if (newStatementRoot == bytes32(0)) revert InvalidRoot();

        original.status = RecordStatus.Amended;

        newMemoryId = _nextMemoryId++;
        records[newMemoryId] = MemoryCommitment({
            signer: msg.sender,
            statementRoot: newStatementRoot,
            contextRoot: newContextRoot,
            consentRoot: newConsentRoot,
            evidenceBundleRoot: newEvidenceRoot,
            parentVersionRoot: original.statementRoot,
            methodologyVersion: methodologyVersion,
            committedAt: uint64(block.timestamp),
            visibility: visibility,
            status: RecordStatus.Active
        });

        rootToMemoryId[newStatementRoot] = newMemoryId;
        versionHistory[originalMemoryId].push(newMemoryId);

        emit MemoryRecordAmended(
            originalMemoryId,
            newMemoryId,
            msg.sender,
            newStatementRoot,
            relation,
            uint64(block.timestamp)
        );
    }

    function retractRecord(
        uint256 memoryId,
        bytes32 retractionRationaleRoot
    ) external {
        MemoryCommitment storage record = records[memoryId];
        if (record.signer == address(0)) revert RecordNotFound(memoryId);
        if (record.signer != msg.sender) revert UnauthorizedSigner(msg.sender, record.signer);

        record.status = RecordStatus.Retracted;

        emit MemoryRecordRetracted(
            memoryId,
            msg.sender,
            retractionRationaleRoot,
            uint64(block.timestamp)
        );
    }

    function getRecord(uint256 memoryId) external view returns (MemoryCommitment memory) {
        MemoryCommitment memory rec = records[memoryId];
        if (rec.signer == address(0)) revert RecordNotFound(memoryId);
        return rec;
    }

    function totalRecords() external view returns (uint256) {
        return _nextMemoryId - 1;
    }
}
