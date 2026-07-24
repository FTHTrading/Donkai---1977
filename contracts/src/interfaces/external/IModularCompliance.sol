// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title  IModularCompliance (thin re-declaration)
/// @notice Minimal interface surface consumed by donkai-core contracts.
///         Full T-REX interface:
///         https://github.com/TokenySolutions/T-REX/blob/main/contracts/compliance/modular/IModularCompliance.sol
/// @dev    Re-declared locally to avoid the full T-REX + ONCHAINID dependency
///         graph (which pins solc =0.8.17). Only the `canTransfer` function
///         is called by `DonkaiFormatWar.stake` when an ERC-20 stake token
///         is configured. Any T-REX-conformant deployment satisfies this ABI.
interface IModularCompliance {
    /// @notice Returns true iff a transfer of `amount` from `_from` to `_to`
    ///         is permitted under the currently bound compliance modules.
    function canTransfer(address _from, address _to, uint256 _amount) external view returns (bool);
}
