// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.24;

/// @notice Shared custom errors for donkai-core contracts.
///         Custom errors are cheaper than string reverts and carry structured payloads.
library DonkaiErrors {
    // ---- Registry ----
    error ZeroMerkleRoot();
    error DuplicateMerkleRoot(bytes32 root, uint256 existingTokenId);
    error IdentityNotVerified(address account);
    error ScoreOutOfRange(uint8 score);
    error EmptyIpfsCid();
    error IpfsCidTooLong(uint256 len);
    error EmptyTitle();
    error TitleTooLong(uint256 len);

    // ---- FormatWar ----
    error MatchupNotFound(uint256 matchupId);
    error MatchupNotOpen(uint256 matchupId);
    error MatchupClosed(uint256 matchupId);
    error MatchupNotClosed(uint256 matchupId);
    error MatchupNotFinalized(uint256 matchupId);
    error MatchupAlreadyFinalized(uint256 matchupId);
    error WindowTooShort();
    error ZeroStake();
    error IncorrectNativeValue(uint256 expected, uint256 got);
    error NativeNotExpected();
    error ComplianceRejected(address from, address to, uint256 amount);
    error NoClaimAvailable();
    error AlreadyClaimed();
    error RefundOnlyWhenCancelled();
    error EthTransferFailed(address to, uint256 amount);
}
