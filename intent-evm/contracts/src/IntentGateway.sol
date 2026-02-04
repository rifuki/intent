// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";

contract IntentGateway is ReentrancyGuard, Ownable, Pausable {
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
    uint256 public protocolFeeBps; // Basis points (e.g. 50 = 0.5%)
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
    event IntentFilled(uint256 indexed intentId, address indexed solver, uint256 outputAmount, uint256 feeAmount);
    event IntentCancelled(uint256 indexed intentId, string reason);
    event ProtocolFeeUpdated(uint256 newFeeBps);
    event TokensRescued(address indexed token, address indexed to, uint256 amount);

    constructor() Ownable(msg.sender) {
        // Initial fee 0%
    }

    function createIntent(
        address inputToken,
        uint256 inputAmount,
        address outputToken,
        uint256 minOutputAmount,
        uint256 deadline
    ) external nonReentrant whenNotPaused returns (uint256 intentId) {
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

    function fillIntent(uint256 intentId, uint256 outputAmount) external nonReentrant whenNotPaused {
        Intent storage intent = intents[intentId];
        
        require(intent.status == Status.Pending, "Not pending");
        require(block.timestamp <= intent.deadline, "Expired");
        require(outputAmount >= intent.minOutputAmount, "Output too low");
        
        intent.status = Status.Filled;
        
        // Transfer output to creator
        IERC20(intent.outputToken).safeTransferFrom(msg.sender, intent.creator, outputAmount);
        
        // Calculate fee
        uint256 feeAmount = 0;
        if (protocolFeeBps > 0) {
            feeAmount = (intent.inputAmount * protocolFeeBps) / 10000;
        }
        
        // Transfer input to solver (minus fee)
        uint256 solverAmount = intent.inputAmount - feeAmount;
        IERC20(intent.inputToken).safeTransfer(msg.sender, solverAmount);
        
        // Keep fee in contract (withdrawable by owner via rescueTokens)
        
        emit IntentFilled(intentId, msg.sender, outputAmount, feeAmount);
    }

    function cancelIntent(uint256 intentId) external nonReentrant {
        Intent storage intent = intents[intentId];
        
        require(intent.status == Status.Pending, "Not pending");
        // require(block.timestamp > intent.deadline, "Not expired yet"); // Allow cancel anytime
        require(msg.sender == intent.creator, "Not creator");
        intent.status = Status.Cancelled;
        IERC20(intent.inputToken).safeTransfer(intent.creator, intent.inputAmount);
        emit IntentCancelled(intentId, "User cancelled");
    }

    // --- Admin Functions ---

    function adminCancelIntent(uint256 intentId) external onlyOwner {
        Intent storage intent = intents[intentId];
        require(intent.status == Status.Pending, "Not pending");
        
        intent.status = Status.Cancelled;
        IERC20(intent.inputToken).safeTransfer(intent.creator, intent.inputAmount);
        emit IntentCancelled(intentId, "Admin cancelled");
    }

    function setProtocolFee(uint256 _feeBps) external onlyOwner {
        require(_feeBps <= 1000, "Fee too high"); // Max 10%
        protocolFeeBps = _feeBps;
        emit ProtocolFeeUpdated(_feeBps);
    }

    function rescueTokens(address token, address to, uint256 amount) external onlyOwner {
        IERC20(token).safeTransfer(to, amount);
        emit TokensRescued(token, to, amount);
    }

    function pause() external onlyOwner {
        _pause();
    }

    function unpause() external onlyOwner {
        _unpause();
    }

    function getIntent(uint256 intentId) external view returns (Intent memory) {
        return intents[intentId];
    }
}
