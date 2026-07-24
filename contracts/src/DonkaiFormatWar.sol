// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.24;

import {IERC20}              from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20}           from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {AccessControl}       from "@openzeppelin/contracts/access/AccessControl.sol";
import {ReentrancyGuard}     from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {Pausable}            from "@openzeppelin/contracts/utils/Pausable.sol";
import {IModularCompliance}  from "./interfaces/external/IModularCompliance.sol";

import {DonkaiErrors}        from "./libraries/DonkaiErrors.sol";

/// @title  DonkaiFormatWar
/// @author Donkai Core Engineering
/// @notice Community-proposed historical tech-rivalry matchups (Betamax vs VHS,
///         Mac vs Wintel, HD-DVD vs Blu-ray, etc.) with 2-sided staking.
///         Winners pull pro-rata from the losing pool via pull-payment.
///         Optional ERC-3643 modular-compliance gate on ERC-20 stakes.
contract DonkaiFormatWar is AccessControl, ReentrancyGuard, Pausable {
    using SafeERC20 for IERC20;

    // -------------------------------------------------------------- roles
    bytes32 public constant PASQUALLY_ORACLE_ROLE = keccak256("PASQUALLY_ORACLE_ROLE");
    bytes32 public constant PAUSER_ROLE           = keccak256("PAUSER_ROLE");
    bytes32 public constant COMPLIANCE_ADMIN_ROLE = keccak256("COMPLIANCE_ADMIN_ROLE");

    // ---------------------------------------------------------- constants
    uint64  public constant MIN_WINDOW    = 3600; // 1 hour minimum voting window
    uint256 public constant MAX_TITLE_LEN = 128;

    enum Side   { A, B }
    enum Status { Open, Finalized, Cancelled }

    struct FormatWarMatchup {
        string  titleA;
        string  titleB;
        address stakeToken;      // address(0) == native $DONKAI (gas token)
        uint64  openedAt;
        uint64  closesAt;
        uint256 totalStakedA;
        uint256 totalStakedB;
        Status  status;
        Side    winner;          // meaningless until status == Finalized
        address proposer;
    }

    IModularCompliance public modularCompliance;
    uint256 public nextMatchupId = 1;

    mapping(uint256 => FormatWarMatchup) private _matchups;
    mapping(uint256 => mapping(address => mapping(Side => uint256))) private _stakes;
    mapping(uint256 => mapping(address => bool)) private _claimed;

    // -------------------------------------------------------------- events
    event MatchupProposed(
        uint256 indexed matchupId, address indexed proposer,
        string titleA, string titleB, address stakeToken, uint64 closesAt
    );
    event Staked(uint256 indexed matchupId, address indexed staker, Side side,
                 uint256 amount, uint256 newSideTotal);
    event MatchupFinalized(uint256 indexed matchupId, Side winner,
                            uint256 totalStakedA, uint256 totalStakedB);
    event MatchupCancelled(uint256 indexed matchupId);
    event PayoutClaimed(uint256 indexed matchupId, address indexed staker, uint256 amount);
    event ComplianceUpdated(address indexed previous, address indexed current);

    // --------------------------------------------------------- constructor
    constructor(address admin, address initialOracle, address initialCompliance) {
        _grantRole(DEFAULT_ADMIN_ROLE,     admin);
        _grantRole(COMPLIANCE_ADMIN_ROLE,  admin);
        _grantRole(PAUSER_ROLE,            admin);
        if (initialOracle != address(0)) _grantRole(PASQUALLY_ORACLE_ROLE, initialOracle);
        modularCompliance = IModularCompliance(initialCompliance);
        emit ComplianceUpdated(address(0), initialCompliance);
    }

    // ----------------------------------------------------------- propose
    function proposeMatchup(
        string calldata titleA,
        string calldata titleB,
        address stakeToken,
        uint64 closesAt
    ) external whenNotPaused returns (uint256 matchupId) {
        uint256 lenA = bytes(titleA).length;
        uint256 lenB = bytes(titleB).length;
        if (lenA == 0) revert DonkaiErrors.EmptyTitle();
        if (lenB == 0) revert DonkaiErrors.EmptyTitle();
        if (lenA > MAX_TITLE_LEN) revert DonkaiErrors.TitleTooLong(lenA);
        if (lenB > MAX_TITLE_LEN) revert DonkaiErrors.TitleTooLong(lenB);
        if (closesAt < block.timestamp + MIN_WINDOW) revert DonkaiErrors.WindowTooShort();

        matchupId = nextMatchupId++;
        _matchups[matchupId] = FormatWarMatchup({
            titleA:       titleA,
            titleB:       titleB,
            stakeToken:   stakeToken,
            openedAt:     uint64(block.timestamp),
            closesAt:     closesAt,
            totalStakedA: 0,
            totalStakedB: 0,
            status:       Status.Open,
            winner:       Side.A,     // ignored until finalized
            proposer:     msg.sender
        });

        emit MatchupProposed(matchupId, msg.sender, titleA, titleB, stakeToken, closesAt);
    }

    // ------------------------------------------------------------ stake
    function stake(uint256 matchupId, Side side, uint256 amount)
        external
        payable
        nonReentrant
        whenNotPaused
    {
        FormatWarMatchup storage m = _matchups[matchupId];
        if (m.openedAt == 0) revert DonkaiErrors.MatchupNotFound(matchupId);
        if (m.status != Status.Open) revert DonkaiErrors.MatchupNotOpen(matchupId);
        if (block.timestamp >= m.closesAt) revert DonkaiErrors.MatchupClosed(matchupId);
        if (amount == 0) revert DonkaiErrors.ZeroStake();

        if (m.stakeToken == address(0)) {
            // Native $DONKAI stake path
            if (msg.value != amount) revert DonkaiErrors.IncorrectNativeValue(amount, msg.value);
        } else {
            if (msg.value != 0) revert DonkaiErrors.NativeNotExpected();
            // Optional ERC-3643 modular-compliance gate for ERC-20 stakes
            if (address(modularCompliance) != address(0)) {
                if (!modularCompliance.canTransfer(msg.sender, address(this), amount)) {
                    revert DonkaiErrors.ComplianceRejected(msg.sender, address(this), amount);
                }
            }
            IERC20(m.stakeToken).safeTransferFrom(msg.sender, address(this), amount);
        }

        _stakes[matchupId][msg.sender][side] += amount;
        uint256 newSideTotal;
        if (side == Side.A) {
            m.totalStakedA += amount;
            newSideTotal = m.totalStakedA;
        } else {
            m.totalStakedB += amount;
            newSideTotal = m.totalStakedB;
        }
        emit Staked(matchupId, msg.sender, side, amount, newSideTotal);
    }

    // --------------------------------------------------------- finalize
    function finalizeMatchup(uint256 matchupId, Side winner)
        external
        onlyRole(PASQUALLY_ORACLE_ROLE)
    {
        FormatWarMatchup storage m = _matchups[matchupId];
        if (m.openedAt == 0) revert DonkaiErrors.MatchupNotFound(matchupId);
        if (m.status != Status.Open) revert DonkaiErrors.MatchupAlreadyFinalized(matchupId);
        if (block.timestamp < m.closesAt) revert DonkaiErrors.MatchupNotClosed(matchupId);

        m.status = Status.Finalized;
        m.winner = winner;
        emit MatchupFinalized(matchupId, winner, m.totalStakedA, m.totalStakedB);
    }

    // ----------------------------------------------------------- cancel
    function cancelMatchup(uint256 matchupId) external onlyRole(DEFAULT_ADMIN_ROLE) {
        FormatWarMatchup storage m = _matchups[matchupId];
        if (m.openedAt == 0) revert DonkaiErrors.MatchupNotFound(matchupId);
        if (m.status != Status.Open) revert DonkaiErrors.MatchupAlreadyFinalized(matchupId);
        m.status = Status.Cancelled;
        emit MatchupCancelled(matchupId);
    }

    // --------------------------------------- claim (winning stakers only)
    function claim(uint256 matchupId) external nonReentrant {
        FormatWarMatchup memory m = _matchups[matchupId];
        if (m.openedAt == 0) revert DonkaiErrors.MatchupNotFound(matchupId);
        if (m.status != Status.Finalized) revert DonkaiErrors.MatchupNotFinalized(matchupId);
        if (_claimed[matchupId][msg.sender]) revert DonkaiErrors.AlreadyClaimed();

        uint256 payout = _computePayout(matchupId, m, msg.sender);
        if (payout == 0) revert DonkaiErrors.NoClaimAvailable();

        _claimed[matchupId][msg.sender] = true;
        _payout(m.stakeToken, msg.sender, payout);
        emit PayoutClaimed(matchupId, msg.sender, payout);
    }

    // --------------------------------------- refund (cancelled matchups)
    function refund(uint256 matchupId) external nonReentrant {
        FormatWarMatchup memory m = _matchups[matchupId];
        if (m.openedAt == 0) revert DonkaiErrors.MatchupNotFound(matchupId);
        if (m.status != Status.Cancelled) revert DonkaiErrors.RefundOnlyWhenCancelled();
        if (_claimed[matchupId][msg.sender]) revert DonkaiErrors.AlreadyClaimed();

        uint256 refundAmount =
            _stakes[matchupId][msg.sender][Side.A] + _stakes[matchupId][msg.sender][Side.B];
        if (refundAmount == 0) revert DonkaiErrors.NoClaimAvailable();

        _claimed[matchupId][msg.sender] = true;
        _payout(m.stakeToken, msg.sender, refundAmount);
        emit PayoutClaimed(matchupId, msg.sender, refundAmount);
    }

    // ------------------------------------------------------------- admin
    function setModularCompliance(address newCompliance) external onlyRole(COMPLIANCE_ADMIN_ROLE) {
        address prev = address(modularCompliance);
        modularCompliance = IModularCompliance(newCompliance);
        emit ComplianceUpdated(prev, newCompliance);
    }

    function pause()   external onlyRole(PAUSER_ROLE) { _pause();   }
    function unpause() external onlyRole(PAUSER_ROLE) { _unpause(); }

    // ------------------------------------------------------------- views
    function getMatchup(uint256 id) external view returns (FormatWarMatchup memory) {
        FormatWarMatchup memory m = _matchups[id];
        if (m.openedAt == 0) revert DonkaiErrors.MatchupNotFound(id);
        return m;
    }

    function stakeOf(uint256 id, address user, Side side) external view returns (uint256) {
        return _stakes[id][user][side];
    }

    function pendingPayout(uint256 id, address user) external view returns (uint256) {
        FormatWarMatchup memory m = _matchups[id];
        if (m.status != Status.Finalized) return 0;
        if (_claimed[id][user]) return 0;
        return _computePayout(id, m, user);
    }

    function hasClaimed(uint256 id, address user) external view returns (bool) {
        return _claimed[id][user];
    }

    // ---------------------------------------------------------- internal
    function _computePayout(uint256 matchupId, FormatWarMatchup memory m, address user)
        internal
        view
        returns (uint256)
    {
        uint256 userWinningStake;
        uint256 winningPool;
        uint256 losingPool;
        if (m.winner == Side.A) {
            userWinningStake = _stakes[matchupId][user][Side.A];
            winningPool      = m.totalStakedA;
            losingPool       = m.totalStakedB;
        } else {
            userWinningStake = _stakes[matchupId][user][Side.B];
            winningPool      = m.totalStakedB;
            losingPool       = m.totalStakedA;
        }
        if (userWinningStake == 0 || winningPool == 0) return 0;
        // principal + pro-rata share of loser pool
        return userWinningStake + (userWinningStake * losingPool) / winningPool;
    }

    function _payout(address token, address to, uint256 amount) internal {
        if (token == address(0)) {
            (bool ok, ) = to.call{value: amount}("");
            if (!ok) revert DonkaiErrors.EthTransferFailed(to, amount);
        } else {
            IERC20(token).safeTransfer(to, amount);
        }
    }
}
