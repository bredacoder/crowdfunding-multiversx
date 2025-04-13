// Import the generated proxy, which allows us to call contract endpoints in tests
use crowdfunding::crowdfunding_proxy;
// Import the Status enum from the generated proxy
use crowdfunding::crowdfunding_proxy::Status;

// Import testing utilities from the MultiversX smart contract scenario framework
use multiversx_sc_scenario::imports::*;

// Path to the compiled smart contract (JSON artifact generated during build)
const CODE_PATH: MxscPath = MxscPath::new("output/crowdfunding.mxsc.json");

// Helper function to set up the blockchain simulation environment
fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // Set the current working directory (helps find files, like the contract code)
    blockchain.set_current_dir_from_workspace("crowdfunding");

    // Register the compiled contract code and its constructor function
    blockchain.register_contract(CODE_PATH, crowdfunding::ContractBuilder);

    blockchain
}

// Test addresses for simulation
// Owner address - represents the contract deployer
const OWNER: TestAddress = TestAddress::new("owner");
// Address where the crowdfunding contract will be deployed
const CROWDFUNDING_ADDRESS: TestSCAddress = TestSCAddress::new("crowdfunding");

// Helper function to deploy the crowdfunding contract
// Returns the world state after deployment
fn crowdfunding_deploy() -> ScenarioWorld {
    let mut world = world();

    // Initialize owner's account with initial balance and nonce
    world.account(OWNER).nonce(0).balance(1000000);

    // Deploy the contract with initial parameters:
    // - target amount: 500_000_000_000
    // - deadline: 123000
    let crowdfunding_address = world
        .tx()
        .from(OWNER)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .init(500_000_000_000u64, 123000u64)
        .code(CODE_PATH)
        .new_address(CROWDFUNDING_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    // Verify the contract was deployed at the expected address
    assert_eq!(crowdfunding_address, CROWDFUNDING_ADDRESS.to_address());

    world
}

#[test]
fn crowdfunding_deploy_test() {
    let mut world = crowdfunding_deploy();

    // Verify owner's balance after deployment
    world.check_account(OWNER).balance(1_000_000);

    // Verify the contract's target amount was set correctly
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .target()
        .returns(ExpectValue(500_000_000_000u64))
        .run();

    // Verify the contract's deadline was set correctly
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .deadline()
        .returns(ExpectValue(123000u64))
        .run();
}

// Test address for a donor
const DONOR: TestAddress = TestAddress::new("donor");

// Helper function to simulate funding the contract
// Returns the world state after funding
fn crowdfunding_fund() -> ScenarioWorld {
    let mut world = crowdfunding_deploy();

    // Initialize donor's account with initial balance
    world.account(DONOR).nonce(0).balance(400_000_000_000u64);

    // Simulate donor funding the contract with 250_000_000_000 EGLD
    world
        .tx()
        .from(DONOR)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .fund()
        .egld(250_000_000_000u64)
        .run();

    world
}

#[test]
fn crowdfunding_fund_test() {
    let mut world = crowdfunding_fund();

    // Verify account states after funding:
    // Check owner's account
    world.check_account(OWNER).nonce(1).balance(1_000_000u64);
    // Check donor's remaining balance (initial - funded amount)
    world
        .check_account(DONOR)
        .nonce(1)
        .balance(150_000_000_000u64);
    // Check contract's balance matches funded amount
    world
        .check_account(CROWDFUNDING_ADDRESS)
        .nonce(0)
        .balance(250_000_000_000u64);

    // Verify contract state after funding:
    // Check target amount hasn't changed
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .target()
        .returns(ExpectValue(500_000_000_000u64))
        .run();
    // Check deadline hasn't changed
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .deadline()
        .returns(ExpectValue(123_000u64))
        .run();
    // Verify donor's deposit was recorded correctly
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .deposit(DONOR)
        .returns(ExpectValue(250_000_000_000u64))
        .run();
}

#[test]
fn crowdfunding_fund_too_late_test() {
    // Set up the initial state by deploying contract and making initial funding
    let mut world = crowdfunding_fund();

    // Set the blockchain timestamp to just after the deadline (123,000 + 1)
    // This simulates the scenario where someone tries to fund after the deadline
    world.current_block().block_timestamp(123_001u64);

    // Attempt to fund the contract with 10 EGLD after the deadline
    world
        .tx()
        .from(DONOR) // Transaction sender is the donor
        .to(CROWDFUNDING_ADDRESS) // Send to the crowdfunding contract
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .fund() // Call the fund() endpoint
        .egld(10_000_000_000u64) // Amount to fund (10 EGLD)
        // Expect the transaction to fail with error code 4 and message
        .with_result(ExpectError(4, "cannot fund after deadline"))
        .run();
    // Status 4 indicates a user error. All errors originating within the contract will return this status.

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .status()
        .returns(ExpectValue(Status::Failed))
        .run();
}
