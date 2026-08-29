// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title DonkaiEvidenceRegistry
 * @author DONK AI Core Protocol Engineering
 * @notice Immutable registry of content-addressed evidence bundle commitments,
 * provenance tiers, source classifications, and AI-editing disclosures.
 */
contract DonkaiEvidenceRegistry {
    enum SourceClass {
        AuthorProvided,
        ThirdPartyWitness,
        InstitutionalArchive,
        PublicRecord,
        MediaBroadcast
    }

    enum EvidenceTier {
        ContemporaneousArtifact,
        SubsequentDocumentation,
        RecollectionSketch,
        DerivativeAnalysis,
        UnverifiedLead
    }

    enum AiDisclosure {
        None,
        RestorationFilterApplied,
        OcrTranscriptionAssisted,
        AiGeneratedIllustrationReference,
        TranslationAssisted
    }

    struct EvidenceItemRef {
        bytes32 contentHash;
        string storageUri;
        SourceClass sourceClass;
        EvidenceTier tier;
        AiDisclosure aiDisclosure;
        uint64 registeredAt;
    }

    struct EvidenceBundleRef {
        uint256 memoryId;
        bytes32 bundleRoot;
        address submitter;
        uint64 committedAt;
        uint32 itemCount;
    }

    uint256 private _nextBundleId = 1;
    mapping(uint256 => EvidenceBundleRef) public bundles;
    mapping(bytes32 => uint256) public rootToBundleId;
    mapping(uint256 => EvidenceItemRef[]) public bundleItems;

    event EvidenceBundleRegistered(
        uint256 indexed bundleId,
        uint256 indexed memoryId,
        bytes32 indexed bundleRoot,
        address submitter,
        uint32 itemCount,
        uint64 committedAt
    );

    event EvidenceItemAppended(
        uint256 indexed bundleId,
        bytes32 indexed contentHash,
        EvidenceTier tier,
        SourceClass sourceClass,
        AiDisclosure aiDisclosure
    );

    error InvalidRoot();
    error BundleAlreadyRegistered(bytes32 root);
    error BundleNotFound(uint256 bundleId);

    function registerEvidenceBundle(
        uint256 memoryId,
        bytes32 bundleRoot,
        bytes32[] calldata itemHashes,
        string[] calldata storageUris,
        SourceClass[] calldata sourceClasses,
        EvidenceTier[] calldata tiers,
        AiDisclosure[] calldata aiDisclosures
    ) external returns (uint256 bundleId) {
        if (bundleRoot == bytes32(0)) revert InvalidRoot();
        if (rootToBundleId[bundleRoot] != 0) revert BundleAlreadyRegistered(bundleRoot);
        require(itemHashes.length == storageUris.length, "Length mismatch");
        require(itemHashes.length == tiers.length, "Length mismatch");

        bundleId = _nextBundleId++;
        bundles[bundleId] = EvidenceBundleRef({
            memoryId: memoryId,
            bundleRoot: bundleRoot,
            submitter: msg.sender,
            committedAt: uint64(block.timestamp),
            itemCount: uint32(itemHashes.length)
        });

        rootToBundleId[bundleRoot] = bundleId;

        for (uint256 i = 0; i < itemHashes.length; i++) {
            bundleItems[bundleId].push(EvidenceItemRef({
                contentHash: itemHashes[i],
                storageUri: storageUris[i],
                sourceClass: sourceClasses[i],
                tier: tiers[i],
                aiDisclosure: aiDisclosures[i],
                registeredAt: uint64(block.timestamp)
            }));

            emit EvidenceItemAppended(
                bundleId,
                itemHashes[i],
                tiers[i],
                sourceClasses[i],
                aiDisclosures[i]
            );
        }

        emit EvidenceBundleRegistered(
            bundleId,
            memoryId,
            bundleRoot,
            msg.sender,
            uint32(itemHashes.length),
            uint64(block.timestamp)
        );
    }

    function getBundleItems(uint256 bundleId) external view returns (EvidenceItemRef[] memory) {
        if (bundles[bundleId].committedAt == 0) revert BundleNotFound(bundleId);
        return bundleItems[bundleId];
    }
}
