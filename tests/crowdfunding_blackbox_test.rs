// Import the generated proxy, which allows us to call contract endpoints in tests
use crowdfunding::crowdfunding_proxy;

// Import testing utilities from the MultiversX smart contract scenario framework
use multiversx_sc_scenario::imports::*;

// Path to the compiled smart contract (JSON artifact generated during build)
const CODE_PATH: MxscPath = MxscPath::new("output/crowdfunding.mxsc.json");

// Creates and initializes the simulated blockchain environment ("world")
fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // Set the current working directory (helps find files, like the contract code)
    blockchain.set_current_dir_from_workspace("crowdfunding");

    // Register the compiled contract code and its constructor function
    blockchain.register_contract(CODE_PATH, crowdfunding::ContractBuilder);

    blockchain
}

// Define a constant test address representing the contract deployer/owner
const OWNER: TestAddress = TestAddress::new("owner");

// Define a constant test address where the contract will be deployed
const CROWDFUNDING_ADDRESS: TestSCAddress = TestSCAddress::new("crowdfunding");

#[test]
fn crowdfunding_deploy_test() {
    // Start a new simulated blockchain world
    let mut world = world();

    // Create a test account with nonce = 0 and balance = 1,000,000
    world.account(OWNER).nonce(0).balance(1_000_000);

    // Deploy the smart contract by sending a transaction from OWNER
    let crowdfunding_address = world
        .tx() // Start a new transaction
        .from(OWNER) // Set sender to OWNER
        .typed(crowdfunding_proxy::CrowdfundingProxy) // Use the generated proxy for type safety
        .init(500_000_000_000u64) // Call the init() function with a target value
        .code(CODE_PATH) // Provide the compiled contract code
        .new_address(CROWDFUNDING_ADDRESS) // Set the expected contract address
        .returns(ReturnsNewAddress) // Expect the function to return the new contract address
        .run(); // Execute the transaction

    // Assert that the deployed address is the one we expected
    assert_eq!(crowdfunding_address, CROWDFUNDING_ADDRESS.to_address());

    // Check that the owner's balance is still 1,000,000 (no funds were deducted in this test setup)
    world.check_account(OWNER).balance(1_000_000);

    // Query the contract's `target()` view function to ensure it returns the correct initial value
    world
        .query() // Start a new read-only call (view)
        .to(CROWDFUNDING_ADDRESS) // Target the deployed contract address
        .typed(crowdfunding_proxy::CrowdfundingProxy) // Use the proxy to access the function
        .target() // Call the `target()` function
        .returns(ExpectValue(500_000_000_000u64)) // Expect it to return 500_000_000_000
        .run(); // Execute the query
}
