// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title DonkaiResearchCredits
 * @author DONK AI Core Protocol Engineering
 * @notice Non-transferable, non-monetary research credits for objectively verifiable protocol milestones.
 *
 * Epistemic & Ethical Rule:
 * MUST NEVER allow forecast markets on historical truth, allegations, trauma, or personal guilt.
 * Allowed ONLY for quantifiable operational milestones (e.g. corroboration count thresholds, translation milestones).
 */
contract DonkaiResearchCredits {
    struct MilestoneQuestion {
        uint256 memoryId;
        string questionText;
        uint32 threshold;
        uint64 deadline;
        bool isResolved;
        bool outcome;
    }

    address public steward;
    uint256 private _nextQuestionId = 1;
    mapping(uint256 => MilestoneQuestion) public questions;
    mapping(address => uint256) public creditBalances;

    event QuestionCreated(uint256 indexed questionId, uint256 memoryId, string question, uint64 deadline);
    event CreditsAwarded(address indexed researcher, uint256 amount);
    event QuestionResolved(uint256 indexed questionId, bool outcome);

    error NonTransferable();
    error OnlySteward();

    modifier onlySteward() {
        if (msg.sender != steward) revert OnlySteward();
        _;
    }

    constructor() {
        steward = msg.sender;
    }

    function createMilestoneQuestion(
        uint256 memoryId,
        string calldata questionText,
        uint32 threshold,
        uint64 deadline
    ) external onlySteward returns (uint256 questionId) {
        questionId = _nextQuestionId++;
        questions[questionId] = MilestoneQuestion({
            memoryId: memoryId,
            questionText: questionText,
            threshold: threshold,
            deadline: deadline,
            isResolved: false,
            outcome: false
        });

        emit QuestionCreated(questionId, memoryId, questionText, deadline);
    }

    function resolveQuestion(uint256 questionId, bool outcome) external onlySteward {
        MilestoneQuestion storage q = questions[questionId];
        require(!q.isResolved, "Already resolved");
        q.isResolved = true;
        q.outcome = outcome;
        emit QuestionResolved(questionId, outcome);
    }

    function awardCredits(address researcher, uint256 amount) external onlySteward {
        creditBalances[researcher] += amount;
        emit CreditsAwarded(researcher, amount);
    }
}
