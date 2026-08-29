// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title DonkaiReviewRegistry
 * @author DONK AI Core Protocol Engineering
 * @notice Transparent ledger for versioned rubrics, human reviewer attestations,
 * bounded historical support classifications, and appeal records.
 */
contract DonkaiReviewRegistry {
    enum SupportClassification {
        HistoricallySupported,
        PartiallySupported,
        ConflictingEvidence,
        Unresolved,
        InsufficientEvidence,
        RetractedByAuthor
    }

    struct ReviewRecord {
        uint256 memoryId;
        address reviewer;
        bytes32 methodologyVersion;
        bytes32 conflictDisclosureRoot;
        bytes32 writtenRationaleRoot;
        SupportClassification classification;
        uint64 reviewedAt;
        uint64 appealWindowEnd;
        bool isAppealed;
    }

    uint256 private _nextReviewId = 1;
    mapping(uint256 => ReviewRecord) public reviews;
    mapping(uint256 => uint256[]) public memoryReviews;

    event ReviewAssessmentCommitted(
        uint256 indexed reviewId,
        uint256 indexed memoryId,
        address indexed reviewer,
        SupportClassification classification,
        bytes32 writtenRationaleRoot,
        uint64 reviewedAt,
        uint64 appealWindowEnd
    );

    event ReviewAppealed(uint256 indexed reviewId, address appellant, bytes32 appealRationaleRoot, uint64 appealedAt);

    function commitAssessment(
        uint256 memoryId,
        bytes32 methodologyVersion,
        bytes32 conflictDisclosureRoot,
        bytes32 writtenRationaleRoot,
        SupportClassification classification,
        uint64 appealDurationSeconds
    ) external returns (uint256 reviewId) {
        reviewId = _nextReviewId++;
        uint64 nowSec = uint64(block.timestamp);

        reviews[reviewId] = ReviewRecord({
            memoryId: memoryId,
            reviewer: msg.sender,
            methodologyVersion: methodologyVersion,
            conflictDisclosureRoot: conflictDisclosureRoot,
            writtenRationaleRoot: writtenRationaleRoot,
            classification: classification,
            reviewedAt: nowSec,
            appealWindowEnd: nowSec + appealDurationSeconds,
            isAppealed: false
        });

        memoryReviews[memoryId].push(reviewId);

        emit ReviewAssessmentCommitted(
            reviewId,
            memoryId,
            msg.sender,
            classification,
            writtenRationaleRoot,
            nowSec,
            nowSec + appealDurationSeconds
        );
    }

    function appealReview(uint256 reviewId, bytes32 appealRationaleRoot) external {
        ReviewRecord storage r = reviews[reviewId];
        require(r.reviewedAt != 0, "Review not found");
        require(block.timestamp <= r.appealWindowEnd, "Appeal window closed");
        require(!r.isAppealed, "Already appealed");

        r.isAppealed = true;
        emit ReviewAppealed(reviewId, msg.sender, appealRationaleRoot, uint64(block.timestamp));
    }

    function getReviewIds(uint256 memoryId) external view returns (uint256[] memory) {
        return memoryReviews[memoryId];
    }
}
