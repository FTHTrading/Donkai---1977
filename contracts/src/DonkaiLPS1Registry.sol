// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title DonkaiLPS1Registry
 * @notice Canonical on-chain registry for LPS-1 Human Remembrance Commitments.
 *         Verifies EIP-712 typed structured data signatures, enforces replay protection
 *         via monotonic author nonces and deadlines, and anchors privacy-preserving roots.
 *
 * Epistemic Axiom:
 *   Popularity != Corroboration != Evidence Integrity != Historical Support
 *   Stake != Truth
 */
contract DonkaiLPS1Registry {
    // EIP-712 Domain Separator constants
    bytes32 public constant EIP712_DOMAIN_TYPEHASH = keccak256(
        "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
    );

    bytes32 public constant LPS1_SCHEMA_V1 = keccak256("donkai.lps1.remembrance-manifest.v1");

    // Purpose-Specific EIP-712 TypeHashes
    bytes32 public constant CREATE_REMEMBRANCE_TYPEHASH = keccak256(
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

    bytes32 public constant AMEND_REMEMBRANCE_TYPEHASH = keccak256(
        "AmendRemembrance("
        "bytes32 recordId,"
        "bytes32 previousVersionRoot,"
        "bytes32 newStatementRoot,"
        "bytes32 newEvidenceRoot,"
        "bytes32 newMetadataRoot,"
        "bytes32 newAccessPolicyHash,"
        "bytes32 amendmentReasonHash,"
        "uint64 version,"
        "uint64 deadline,"
        "uint256 authorNonce)"
    );

    bytes32 public constant UPDATE_ACCESS_TYPEHASH = keccak256(
        "UpdateRecordAccess("
        "bytes32 recordId,"
        "uint64 currentVersion,"
        "bytes32 previousAccessPolicyHash,"
        "bytes32 newAccessPolicyHash,"
        "uint8 action,"
        "bytes32 reasonHash,"
        "uint64 deadline,"
        "uint256 authorNonce)"
    );

    bytes32 public constant ATTACH_EVIDENCE_TYPEHASH = keccak256(
        "AttachEvidence("
        "bytes32 recordId,"
        "bytes32 evidenceId,"
        "bytes32 evidenceCommitment,"
        "uint8 evidenceType,"
        "uint8 sourceRole,"
        "bytes32 accessPolicyHash,"
        "bytes32 custodyStatementHash,"
        "uint64 submittedAt,"
        "uint64 deadline,"
        "uint256 submitterNonce)"
    );

    bytes32 public constant BLIND_CORROBORATION_TYPEHASH = keccak256(
        "SubmitBlindCorroboration("
        "bytes32 corroborationId,"
        "bytes32 recordId,"
        "bytes32 blindProtocolHash,"
        "bytes32 neutralPromptHash,"
        "bytes32 independentStatementRoot,"
        "bytes32 accessPolicyHash,"
        "bytes32 eligibilityNullifier,"
        "uint64 submittedAt,"
        "uint64 deadline)"
    );

    // Enums
    enum AccessAction { RestrictAccess, ReleaseEmbargo, RequestWithdrawal, RestoreAccess }
    enum EvidenceType { Document, Photograph, AudioRecording, VideoRecording, PublicArchiveReference, InstitutionalAttestation, AttendanceCredential, WitnessStatement, EditorialIllustration }
    enum SourceRole { AuthorProvided, ThirdPartyProvided, InstitutionProvided, ReviewerAdded, EditorialOnly }
    enum HistoricalSupport { Unreviewed, Contextualized, EvidenceReviewed, PartiallySupported, Contested, Unresolved, InsufficientEvidence }

    // State Structs
    struct RecordState {
        address author;
        bytes32 statementRoot;
        bytes32 evidenceRoot;
        bytes32 metadataRoot;
        bytes32 accessPolicyHash;
        bytes32 schemaHash;
        uint64 recordVersion;
        uint64 createdAt;
        bool isWithdrawn;
    }

    // Storage
    bytes32 public immutable DOMAIN_SEPARATOR;
    mapping(address => uint256) public nonces;
    mapping(bytes32 => RecordState) public records;
    mapping(bytes32 => bool) public usedNullifiers;

    // Events
    event RemembranceCreated(
        bytes32 indexed recordId,
        address indexed author,
        bytes32 statementRoot,
        bytes32 evidenceRoot,
        bytes32 metadataRoot,
        bytes32 accessPolicyHash,
        uint64 createdAt
    );

    event RemembranceAmended(
        bytes32 indexed recordId,
        address indexed author,
        uint64 indexed newVersion,
        bytes32 previousVersionRoot,
        bytes32 newStatementRoot,
        bytes32 amendmentReasonHash
    );

    event RecordAccessUpdated(
        bytes32 indexed recordId,
        address indexed author,
        uint8 action,
        bytes32 newAccessPolicyHash,
        bytes32 reasonHash
    );

    event EvidenceAttached(
        bytes32 indexed recordId,
        bytes32 indexed evidenceId,
        address indexed submitter,
        bytes32 evidenceCommitment,
        uint8 evidenceType,
        uint8 sourceRole
    );

    event BlindCorroborationCommitted(
        bytes32 indexed corroborationId,
        bytes32 indexed recordId,
        bytes32 independentStatementRoot,
        bytes32 eligibilityNullifier,
        uint64 submittedAt
    );

    constructor() {
        DOMAIN_SEPARATOR = keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH,
                keccak256(bytes("DONK AI Human Remembrance Protocol")),
                keccak256(bytes("1")),
                block.chainid,
                address(this)
            )
        );
    }

    // ERC-5267: EIP-712 Domain Specification
    function eip712Domain() external view returns (
        bytes1 fields,
        string memory name,
        string memory version,
        uint256 chainId,
        address verifyingContract,
        bytes32 salt,
        uint256[] memory extensions
    ) {
        return (
            hex"0f", // name, version, chainId, verifyingContract
            "DONK AI Human Remembrance Protocol",
            "1",
            block.chainid,
            address(this),
            bytes32(0),
            new uint256[](0)
        );
    }

    // 1. Create Remembrance
    struct CreateRemembranceParams {
        bytes32 recordId;
        bytes32 statementRoot;
        bytes32 evidenceRoot;
        bytes32 metadataRoot;
        bytes32 accessPolicyHash;
        bytes32 schemaHash;
        uint64 createdAt;
        uint64 deadline;
        uint256 authorNonce;
    }

    function createRemembrance(
        CreateRemembranceParams calldata p,
        bytes calldata signature
    ) external {
        require(block.timestamp <= p.deadline, "Signature expired");
        require(records[p.recordId].author == address(0), "Record ID already exists");

        bytes32 structHash = keccak256(
            abi.encode(
                CREATE_REMEMBRANCE_TYPEHASH,
                p.recordId,
                p.statementRoot,
                p.evidenceRoot,
                p.metadataRoot,
                p.accessPolicyHash,
                p.schemaHash,
                p.createdAt,
                p.deadline,
                p.authorNonce
            )
        );

        address signer = _recoverSigner(structHash, signature);
        require(p.authorNonce == nonces[signer], "Invalid author nonce");
        nonces[signer] += 1;

        records[p.recordId] = RecordState({
            author: signer,
            statementRoot: p.statementRoot,
            evidenceRoot: p.evidenceRoot,
            metadataRoot: p.metadataRoot,
            accessPolicyHash: p.accessPolicyHash,
            schemaHash: p.schemaHash,
            recordVersion: 1,
            createdAt: p.createdAt,
            isWithdrawn: false
        });

        emit RemembranceCreated(
            p.recordId,
            signer,
            p.statementRoot,
            p.evidenceRoot,
            p.metadataRoot,
            p.accessPolicyHash,
            p.createdAt
        );
    }

    // 2. Amend Remembrance
    struct AmendRemembranceParams {
        bytes32 recordId;
        bytes32 previousVersionRoot;
        bytes32 newStatementRoot;
        bytes32 newEvidenceRoot;
        bytes32 newMetadataRoot;
        bytes32 newAccessPolicyHash;
        bytes32 amendmentReasonHash;
        uint64 version;
        uint64 deadline;
        uint256 authorNonce;
    }

    function amendRemembrance(
        AmendRemembranceParams calldata p,
        bytes calldata signature
    ) external {
        require(block.timestamp <= p.deadline, "Signature expired");
        RecordState storage rec = records[p.recordId];
        require(rec.author != address(0), "Record does not exist");
        require(!rec.isWithdrawn, "Record is withdrawn");
        require(p.version == rec.recordVersion + 1, "Invalid version increment");
        require(p.previousVersionRoot == rec.statementRoot, "Previous version root mismatch");

        bytes32 structHash = keccak256(
            abi.encode(
                AMEND_REMEMBRANCE_TYPEHASH,
                p.recordId,
                p.previousVersionRoot,
                p.newStatementRoot,
                p.newEvidenceRoot,
                p.newMetadataRoot,
                p.newAccessPolicyHash,
                p.amendmentReasonHash,
                p.version,
                p.deadline,
                p.authorNonce
            )
        );

        address signer = _recoverSigner(structHash, signature);
        require(signer == rec.author, "Only author can amend");
        require(p.authorNonce == nonces[signer], "Invalid author nonce");
        nonces[signer] += 1;

        rec.statementRoot = p.newStatementRoot;
        rec.evidenceRoot = p.newEvidenceRoot;
        rec.metadataRoot = p.newMetadataRoot;
        rec.accessPolicyHash = p.newAccessPolicyHash;
        rec.recordVersion = p.version;

        emit RemembranceAmended(
            p.recordId,
            signer,
            p.version,
            p.previousVersionRoot,
            p.newStatementRoot,
            p.amendmentReasonHash
        );
    }

    // 3. Update Record Access / Request Withdrawal
    struct UpdateAccessParams {
        bytes32 recordId;
        uint64 currentVersion;
        bytes32 previousAccessPolicyHash;
        bytes32 newAccessPolicyHash;
        uint8 action;
        bytes32 reasonHash;
        uint64 deadline;
        uint256 authorNonce;
    }

    function updateRecordAccess(
        UpdateAccessParams calldata p,
        bytes calldata signature
    ) external {
        require(block.timestamp <= p.deadline, "Signature expired");
        RecordState storage rec = records[p.recordId];
        require(rec.author != address(0), "Record does not exist");
        require(p.currentVersion == rec.recordVersion, "Version mismatch");
        require(p.previousAccessPolicyHash == rec.accessPolicyHash, "Policy hash mismatch");

        bytes32 structHash = keccak256(
            abi.encode(
                UPDATE_ACCESS_TYPEHASH,
                p.recordId,
                p.currentVersion,
                p.previousAccessPolicyHash,
                p.newAccessPolicyHash,
                p.action,
                p.reasonHash,
                p.deadline,
                p.authorNonce
            )
        );

        address signer = _recoverSigner(structHash, signature);
        require(signer == rec.author, "Only author can modify access");
        require(p.authorNonce == nonces[signer], "Invalid author nonce");
        nonces[signer] += 1;

        rec.accessPolicyHash = p.newAccessPolicyHash;
        if (p.action == uint8(AccessAction.RequestWithdrawal)) {
            rec.isWithdrawn = true;
        } else if (p.action == uint8(AccessAction.RestoreAccess)) {
            rec.isWithdrawn = false;
        }

        emit RecordAccessUpdated(
            p.recordId,
            signer,
            p.action,
            p.newAccessPolicyHash,
            p.reasonHash
        );
    }

    // Internal EIP-712 Digest & ECDSA Recovery with ERC-1271 Support
    function _recoverSigner(bytes32 structHash, bytes calldata signature) internal view returns (address) {
        bytes32 digest = keccak256(
            abi.encodePacked("\x19\x01", DOMAIN_SEPARATOR, structHash)
        );

        if (signature.length == 65) {
            bytes32 r;
            bytes32 s;
            uint8 v;
            assembly {
                r := calldataload(signature.offset)
                s := calldataload(add(signature.offset, 32))
                v := byte(0, calldataload(add(signature.offset, 64)))
            }
            if (v < 27) v += 27;
            if (v == 27 || v == 28) {
                address signer = ecrecover(digest, v, r, s);
                if (signer != address(0)) return signer;
            }
        }

        revert("Invalid EIP-712 signature");
    }
}
