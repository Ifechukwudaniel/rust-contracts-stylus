// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "../../solidity-contracts/contracts/interfaces/IERC3156FlashBorrower.sol";
import "../../solidity-contracts/contracts/token/ERC20/IERC20.sol";
import "../../solidity-contracts/contracts/utils/Address.sol";

// Needed for benches, see [`benches/src/erc20_flash_mint.rs`]
/**
 * @dev WARNING: this IERC3156FlashBorrower mock implementation is for testing purposes ONLY.
 * Writing a secure flash lock borrower is not an easy task, and should be done with the utmost care.
 * This is not an example of how it should be done, and no pattern present in this mock should be considered secure.
 * Following best practices, always have your contract properly audited before using them to manipulate important funds on
 * live networks.
 */
contract ERC3156FlashBorrowerMock is IERC3156FlashBorrower {
    bytes32 internal constant _RETURN_VALUE =
        keccak256("ERC3156FlashBorrower.onFlashLoan");

    bool _enableApprove;
    bool _validReturn;

    event BalanceOf(address token, address account, uint256 value);
    event TotalSupply(address token, uint256 value);

    constructor(bool validReturn, bool enableApprove) {
        _enableApprove = enableApprove;
        _validReturn = validReturn;
    }

    function onFlashLoan(
        address /* initiator */,
        address token,
        uint256 amount,
        uint256 fee,
        bytes calldata data
    ) public returns (bytes32) {
        require(msg.sender == token, "Invalid token address");

        emit BalanceOf(
            token,
            address(this),
            IERC20(token).balanceOf(address(this))
        );

        emit TotalSupply(token, IERC20(token).totalSupply());

        if (data.length > 0) {
            // WARNING: This code is for testing purposes only! Do not use in production.
            Address.functionCall(token, data);
        }

        if (_enableApprove) {
            IERC20(token).approve(token, amount + fee);
        }

        return _validReturn ? _RETURN_VALUE : bytes32(0);
    }
}