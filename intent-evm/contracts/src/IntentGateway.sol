// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

contract IntentGateway is ReentrancyGuard {
    using SafeERC20 for IERC20;

    enum Status {
        Pending,    // 0 - Waiting for solver
        Filled,     // 1 - Successfully filled
        Cancelled   // 2 - Cancelled by user
    }

    struct Intent {
        address creator;
        address inputToken;
        uint256 inputAmount;
        address outputToken;
        uint256 minOutputAmount;
        uint256 deadline;
        Status status;

    }

    uint256 public nextIntentId;
    mapping(uint256 => Intent) public intents;

    event IntentCreated(
        uint256 indexed intentId,
        address indexed creator,
        address inputToken,
        uint256 inputAmount,
        address outputToken,
        uint256 minOutputAmount,
        uint256 deadline
    );
    event IntentFilled(uint256 indexed intentId, address indexed solver, uint256 outputAmount);
    event IntentCancelled(uint256 indexed intentId);

    function createIntent(
        address inputToken,
        uint256 inputAmount,
        address outputToken,
        uint256 minOutputAmount,
        uint256 deadline
    ) external nonReentrant returns (uint256 intentId) {
        require(deadline > block.timestamp, "Invalid deadline");
        require(inputAmount > 0, "Invalid input amount");
        require(minOutputAmount > 0, "Invalid min output");
        require(inputToken != outputToken, "Same token");
        intentId = nextIntentId++;
        IERC20(inputToken).safeTransferFrom(msg.sender, address(this), inputAmount);
        intents[intentId] = Intent({
            creator: msg.sender,
            inputToken: inputToken,
            inputAmount: inputAmount,
            outputToken: outputToken,
            minOutputAmount: minOutputAmount,
            deadline: deadline,
            status: Status.Pending
        });
        emit IntentCreated(intentId, msg.sender, inputToken, inputAmount, outputToken, minOutputAmount, deadline);
    }

    function fillIntent(uint256 intentId, uint256 outputAmount) external nonReentrant {
        Intent storage intent = intents[intentId];
        
        require(intent.status == Status.Pending, "Not pending");
        require(block.timestamp <= intent.deadline, "Expired");
        require(outputAmount >= intent.minOutputAmount, "Output too low");
        intent.status = Status.Filled;
        IERC20(intent.outputToken).safeTransferFrom(msg.sender, intent.creator, outputAmount);
        IERC20(intent.inputToken).safeTransfer(msg.sender, intent.inputAmount);
        emit IntentFilled(intentId, msg.sender, outputAmount);
    }

    function cancelIntent(uint256 intentId) external nonReentrant {
        Intent storage intent = intents[intentId];
        
        require(intent.status == Status.Pending, "Not pending");
        require(block.timestamp > intent.deadline, "Not expired yet");
        require(msg.sender == intent.creator, "Not creator");
        intent.status = Status.Cancelled;
        IERC20(intent.inputToken).safeTransfer(intent.creator, intent.inputAmount);
        emit IntentCancelled(intentId);
    }

    function getIntent(uint256 intentId) external view returns (Intent memory) {
        return intents[intentId];
    }
}
