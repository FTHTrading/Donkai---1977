// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title  IIdentityRegistry (thin re-declaration)
/// @notice Minimal interface surface consumed by donkai-core contracts.
///         Full T-REX interface:
///         https://github.com/TokenySolutions/T-REX/blob/main/contracts/registry/interface/IIdentityRegistry.sol
/// @dev    Re-declared locally to avoid the full T-REX + ONCHAINID dependency
///         graph (which pins solc =0.8.17 and is incompatible with
///         OpenZeppelin v5's ^0.8.20 requirement). Only the `isVerified`
///         function is called by `DonkaiTimelineRegistry.mintRelic`.
///         Any T-REX-conformant deployment satisfies this ABI.
interface IIdentityRegistry {
    /// @notice Returns true iff `userAddress` has a verified ONCHAINID that
    ///         satisfies all required claim topics from the trusted issuers.
    function isVerified(address userAddress) external view returns (bool);
}
