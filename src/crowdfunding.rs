// This directive disables the Rust standard library (required for smart contracts on blockchains)
#![no_std]

// Ignore warnings for unused imports (mostly for development convenience)
#[allow(unused_imports)]
use multiversx_sc::imports::*;

// Import the generated proxy (used for testing and interaction, not directly used here)
pub mod crowdfunding_proxy;

/// This is the actual contract definition
#[multiversx_sc::contract]
pub trait Crowdfunding {
    /// The `init` function runs only once—at contract deployment.
    /// It takes a target amount (e.g., funding goal) and saves it in storage.
    #[init]
    fn init(&self, target: BigUint) {
        // Store the value in contract storage under the key "target"
        self.target().set(target);
    }

    /// This is a placeholder function for upgrading the contract in the future.
    /// It doesn’t do anything yet but is required for upgradable contracts.
    #[upgrade]
    fn upgrade(&self) {}

    /// A view (read-only) function that lets anyone query the stored target value.
    #[view]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;
}
