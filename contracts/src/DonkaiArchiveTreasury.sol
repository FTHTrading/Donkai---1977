// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title DonkaiArchiveTreasury
 * @author DONK AI Core Protocol Engineering
 * @notice Protocol treasury for non-factual grants, archival storage subsidies, and translation incentives.
 */
contract DonkaiArchiveTreasury {
    struct Grant {
        uint256 grantId;
        address recipient;
        uint256 amount;
        string purpose;
        bool isDisbursed;
        uint64 approvedAt;
    }

    address public steward;
    uint256 private _nextGrantId = 1;
    mapping(uint256 => Grant) public grants;

    event GrantCreated(uint256 indexed grantId, address indexed recipient, uint256 amount, string purpose);
    event GrantDisbursed(uint256 indexed grantId, address indexed recipient, uint256 amount);

    error OnlySteward();

    modifier onlySteward() {
        if (msg.sender != steward) revert OnlySteward();
        _;
    }

    constructor() {
        steward = msg.sender;
    }

    receive() external payable {}

    function proposeGrant(address recipient, uint256 amount, string calldata purpose) external onlySteward returns (uint256 grantId) {
        grantId = _nextGrantId++;
        grants[grantId] = Grant({
            grantId: grantId,
            recipient: recipient,
            amount: amount,
            purpose: purpose,
            isDisbursed: false,
            approvedAt: uint64(block.timestamp)
        });

        emit GrantCreated(grantId, recipient, amount, purpose);
    }

    function disburseGrant(uint256 grantId) external onlySteward {
        Grant storage g = grants[grantId];
        require(!g.isDisbursed, "Grant already disbursed");
        require(address(this).balance >= g.amount, "Insufficient treasury balance");

        g.isDisbursed = true;
        (bool ok, ) = g.recipient.call{value: g.amount}("");
        require(ok, "Disbursement failed");

        emit GrantDisbursed(grantId, g.recipient, g.amount);
    }
}
