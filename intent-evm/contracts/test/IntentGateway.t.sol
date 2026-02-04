// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console2} from "forge-std/Test.sol";
import {IntentGateway} from "../src/IntentGateway.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/// @notice Mock ERC20 for testing
contract MockERC20 is ERC20 {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) {}

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract IntentGatewayTest is Test {
    IntentGateway public gateway;
    MockERC20 public usdc;
    MockERC20 public weth;

    address public alice = makeAddr("alice");
    address public bob = makeAddr("bob"); // solver
    address public charlie = makeAddr("charlie");

    uint256 constant USDC_DECIMALS = 6;
    uint256 constant WETH_DECIMALS = 18;

    function setUp() public {
        // Deploy contracts
        gateway = new IntentGateway();
        usdc = new MockERC20("USD Coin", "USDC");
        weth = new MockERC20("Wrapped Ether", "WETH");

        // Mint tokens to users
        usdc.mint(alice, 10_000 * 10 ** USDC_DECIMALS);   // 10,000 USDC
        usdc.mint(charlie, 10_000 * 10 ** USDC_DECIMALS); // 10,000 USDC
        weth.mint(bob, 100 * 10 ** WETH_DECIMALS);         // 100 WETH (solver)

        // Approve gateway from users
        vm.prank(alice);
        usdc.approve(address(gateway), type(uint256).max);

        vm.prank(charlie);
        usdc.approve(address(gateway), type(uint256).max);

        vm.prank(bob);
        weth.approve(address(gateway), type(uint256).max);
    }

    /*//////////////////////////////////////////////////////////////
                        CREATE INTENT TESTS
    //////////////////////////////////////////////////////////////*/

    function test_CreateIntent_Success() public {
        uint256 inputAmount = 100 * 10 ** USDC_DECIMALS; // 100 USDC
        uint256 minOutput = 45 * 10 ** (WETH_DECIMALS - 3); // 0.045 WETH
        uint256 deadline = block.timestamp + 1 hours;

        vm.prank(alice);
        uint256 intentId = gateway.createIntent(
            address(usdc),
            inputAmount,
            address(weth),
            minOutput,
            deadline
        );

        assertEq(intentId, 0, "First intent should have ID 0");
        assertEq(gateway.nextIntentId(), 1, "Next intent ID should be 1");

        // Check intent was stored correctly
        IntentGateway.Intent memory intent = gateway.getIntent(intentId);
        assertEq(intent.creator, alice);
        assertEq(intent.inputToken, address(usdc));
        assertEq(intent.inputAmount, inputAmount);
        assertEq(intent.outputToken, address(weth));
        assertEq(intent.minOutputAmount, minOutput);
        assertEq(intent.deadline, deadline);
        assertEq(uint8(intent.status), 0); // Pending

        // Check tokens were transferred
        assertEq(usdc.balanceOf(address(gateway)), inputAmount);
    }

    function test_CreateIntent_MultipleIntents() public {
        uint256 deadline = block.timestamp + 1 hours;

        vm.prank(alice);
        uint256 id1 = gateway.createIntent(
            address(usdc), 100 * 10 ** USDC_DECIMALS,
            address(weth), 40e15, deadline // 0.04 WETH
        );

        vm.prank(charlie);
        uint256 id2 = gateway.createIntent(
            address(usdc), 200 * 10 ** USDC_DECIMALS,
            address(weth), 80e15, deadline // 0.08 WETH
        );

        assertEq(id1, 0);
        assertEq(id2, 1);
        assertEq(gateway.nextIntentId(), 2);
    }

    function test_CreateIntent_RevertInvalidDeadline() public {
        vm.prank(alice);
        vm.expectRevert("Invalid deadline");
        gateway.createIntent(
            address(usdc), 100e6,
            address(weth), 1 ether,
            block.timestamp // deadline is now, not future
        );
    }

    function test_CreateIntent_RevertZeroInputAmount() public {
        vm.prank(alice);
        vm.expectRevert("Invalid input amount");
        gateway.createIntent(
            address(usdc), 0,
            address(weth), 1 ether,
            block.timestamp + 1 hours
        );
    }

    function test_CreateIntent_RevertZeroMinOutput() public {
        vm.prank(alice);
        vm.expectRevert("Invalid min output");
        gateway.createIntent(
            address(usdc), 100e6,
            address(weth), 0,
            block.timestamp + 1 hours
        );
    }

    function test_CreateIntent_RevertSameToken() public {
        vm.prank(alice);
        vm.expectRevert("Same token");
        gateway.createIntent(
            address(usdc), 100e6,
            address(usdc), 100e6,
            block.timestamp + 1 hours
        );
    }

    /*//////////////////////////////////////////////////////////////
                        FILL INTENT TESTS
    //////////////////////////////////////////////////////////////*/

    function test_FillIntent_Success() public {
        // Alice creates intent
        uint256 inputAmount = 100 * 10 ** USDC_DECIMALS;
        uint256 minOutput = 45 * 10 ** (WETH_DECIMALS - 3); // 0.045 WETH
        uint256 deadline = block.timestamp + 1 hours;

        vm.prank(alice);
        uint256 intentId = gateway.createIntent(
            address(usdc), inputAmount,
            address(weth), minOutput, deadline
        );

        uint256 aliceWethBefore = weth.balanceOf(alice);
        uint256 bobUsdcBefore = usdc.balanceOf(bob);
        uint256 bobWethBefore = weth.balanceOf(bob);

        // Bob fills with exact minimum
        vm.prank(bob);
        gateway.fillIntent(intentId, minOutput);

        // Check alice received WETH
        assertEq(weth.balanceOf(alice), aliceWethBefore + minOutput);

        // Check bob received USDC and paid WETH
        assertEq(usdc.balanceOf(bob), bobUsdcBefore + inputAmount);
        assertEq(weth.balanceOf(bob), bobWethBefore - minOutput);

        // Check intent status
        IntentGateway.Intent memory intent = gateway.getIntent(intentId);
        assertEq(uint8(intent.status), 1); // Filled
    }

    function test_FillIntent_MoreThanMinimum() public {
        uint256 inputAmount = 100 * 10 ** USDC_DECIMALS;
        uint256 minOutput = 45 * 10 ** (WETH_DECIMALS - 3);
        uint256 actualOutput = 50 * 10 ** (WETH_DECIMALS - 3); // More than min

        vm.prank(alice);
        uint256 intentId = gateway.createIntent(
            address(usdc), inputAmount,
            address(weth), minOutput,
            block.timestamp + 1 hours
        );

        uint256 aliceWethBefore = weth.balanceOf(alice);

        vm.prank(bob);
        gateway.fillIntent(intentId, actualOutput);

        // Alice should receive the actual (higher) amount
        assertEq(weth.balanceOf(alice), aliceWethBefore + actualOutput);
    }

    function test_FillIntent_RevertOutputTooLow() public {
        uint256 minOutput = 50e15; // 0.05 WETH

        vm.prank(alice);
        uint256 intentId = gateway.createIntent(
            address(usdc), 100e6,
            address(weth), minOutput,
            block.timestamp + 1 hours
        );

        vm.prank(bob);
        vm.expectRevert("Output too low");
        gateway.fillIntent(intentId, minOutput - 1);
    }

    function test_FillIntent_RevertExpired() public {
        vm.prank(alice);
        uint256 intentId = gateway.createIntent(
            address(usdc), 100e6,
            address(weth), 50e15,
            block.timestamp + 1 hours
        );

        // Fast forward past deadline
        vm.warp(block.timestamp + 2 hours);

        vm.prank(bob);
        vm.expectRevert("Expired");
        gateway.fillIntent(intentId, 50e15);
    }

    function test_FillIntent_RevertAlreadyFilled() public {
        vm.prank(alice);
        uint256 intentId = gateway.createIntent(
            address(usdc), 100e6,
            address(weth), 50e15,
            block.timestamp + 1 hours
        );

        // First fill succeeds
        vm.prank(bob);
        gateway.fillIntent(intentId, 50e15);

        // Second fill should fail
        vm.prank(bob);
        vm.expectRevert("Not pending");
        gateway.fillIntent(intentId, 50e15);
    }

    /*//////////////////////////////////////////////////////////////
                        CANCEL INTENT TESTS
    //////////////////////////////////////////////////////////////*/

    function test_CancelIntent_Success() public {
        uint256 inputAmount = 100 * 10 ** USDC_DECIMALS;
        uint256 aliceUsdcBefore = usdc.balanceOf(alice);

        vm.prank(alice);
        uint256 intentId = gateway.createIntent(
            address(usdc), inputAmount,
            address(weth), 50e15,
            block.timestamp + 1 hours
        );

        // Tokens should be in gateway
        assertEq(usdc.balanceOf(alice), aliceUsdcBefore - inputAmount);

        // Fast forward past deadline
        vm.warp(block.timestamp + 2 hours);

        vm.prank(alice);
        gateway.cancelIntent(intentId);

        // Alice should get tokens back
        assertEq(usdc.balanceOf(alice), aliceUsdcBefore);

        // Intent should be cancelled
        IntentGateway.Intent memory intent = gateway.getIntent(intentId);
        assertEq(uint8(intent.status), 2); // Cancelled
    }

    function test_AdminCancel_Success() public {
        uint256 aliceUsdcBefore = usdc.balanceOf(alice);
        
        vm.prank(alice);
        uint256 intentId = gateway.createIntent(
            address(usdc), 100e6,
            address(weth), 50e15,
            block.timestamp + 1 hours
        );

        // Owner (this contract) cancels
        gateway.adminCancelIntent(intentId);

        IntentGateway.Intent memory intent = gateway.getIntent(intentId);
        assertEq(uint8(intent.status), 2); // Cancelled
        assertEq(usdc.balanceOf(alice), aliceUsdcBefore); // Refunded fully
    }

    function test_ProtocolFee_Success() public {
        gateway.setProtocolFee(100); // 1%

        vm.prank(alice);
        uint256 intentId = gateway.createIntent(
            address(usdc), 100e6,
            address(weth), 50e15,
            block.timestamp + 1 hours
        );

        uint256 solverUsdcBefore = usdc.balanceOf(bob);
        
        vm.prank(bob);
        weth.approve(address(gateway), 50e15);
        vm.prank(bob);
        gateway.fillIntent(intentId, 50e15);

        // Check fee deduction
        // Input: 100 USDC. Fee 1% = 1 USDC. Solver gets 99 USDC.
        assertEq(usdc.balanceOf(bob), solverUsdcBefore + 99e6);
        assertEq(usdc.balanceOf(address(gateway)), 1e6); // Fee in contract
    }

    function test_RescueTokens_Success() public {
        // Send tokens to gateway (simulated stuck fee or accidental send)
        usdc.mint(address(gateway), 500e6);

        // Rescue
        gateway.rescueTokens(address(usdc), address(this), 500e6);
        assertEq(usdc.balanceOf(address(this)), 500e6);
    }
    
    function test_Pause_PreventsCreation() public {
        gateway.pause();
        
        vm.prank(alice);
        vm.expectRevert(); // Enforced by Pausable
        gateway.createIntent(address(usdc), 100e6, address(weth), 50e15, block.timestamp + 1 hours);
    }

    function test_CancelIntent_RevertNotCreator() public {
        vm.prank(alice);
        uint256 intentId = gateway.createIntent(
            address(usdc), 100e6,
            address(weth), 50e15,
            block.timestamp + 1 hours
        );

        vm.warp(block.timestamp + 2 hours);

        // Bob tries to cancel Alice's intent
        vm.prank(bob);
        vm.expectRevert("Not creator");
        gateway.cancelIntent(intentId);
    }

    function test_CancelIntent_RevertAlreadyFilled() public {
        vm.prank(alice);
        uint256 intentId = gateway.createIntent(
            address(usdc), 100e6,
            address(weth), 50e15,
            block.timestamp + 1 hours
        );

        // Bob fills it
        vm.prank(bob);
        gateway.fillIntent(intentId, 50e15);

        vm.warp(block.timestamp + 2 hours);

        // Alice tries to cancel filled intent
        vm.prank(alice);
        vm.expectRevert("Not pending");
        gateway.cancelIntent(intentId);
    }

    /*//////////////////////////////////////////////////////////////
                        EDGE CASES
    //////////////////////////////////////////////////////////////*/

    function test_GetIntent_NonExistent() public view {
        IntentGateway.Intent memory intent = gateway.getIntent(999);
        // Should return default values
        assertEq(intent.creator, address(0));
        assertEq(intent.inputAmount, 0);
    }

    function testFuzz_CreateIntent_VariableAmounts(
        uint256 inputAmount,
        uint256 minOutputAmount
    ) public {
        inputAmount = bound(inputAmount, 1, 10_000 * 10 ** USDC_DECIMALS);
        minOutputAmount = bound(minOutputAmount, 1, 100 * 10 ** WETH_DECIMALS);

        vm.prank(alice);
        uint256 intentId = gateway.createIntent(
            address(usdc), inputAmount,
            address(weth), minOutputAmount,
            block.timestamp + 1 hours
        );

        IntentGateway.Intent memory intent = gateway.getIntent(intentId);
        assertEq(intent.inputAmount, inputAmount);
        assertEq(intent.minOutputAmount, minOutputAmount);
    }
}
