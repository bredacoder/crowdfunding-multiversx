#![no_std] // Indicates that this code doesn't use the Rust standard library, which is common for blockchain contracts

use multiversx_sc::{derive_imports::*, imports::*}; // Import necessary MultiversX smart contract libraries
pub mod crowdfunding_proxy; // Public module for contract interaction proxies

// Enum representing the possible states of the crowdfunding campaign
#[type_abi] // Makes this type visible in the ABI (Application Binary Interface)
#[derive(TopEncode, TopDecode, PartialEq, Clone, Copy)] // Implements serialization and comparison traits
pub enum Status {
    FundingPeriod, // Campaign is still open for funding
    Successful,    // Campaign reached its target before deadline
    Failed,        // Deadline passed and target wasn't reached
}

#[multiversx_sc::contract] // Marks this trait as a MultiversX smart contract
pub trait Crowdfunding {
    // Constructor function, called once when the contract is deployed
    #[init]
    fn init(&self, target: BigUint, deadline: u64) {
        require!(target > 0, "Target must be more than 0"); // Validate that target amount is positive
        self.target().set(target); // Store the funding target

        require!(
            deadline > self.get_current_time(),
            "Deadline can't be in the past"
        ); // Validate that deadline is in the future
        self.deadline().set(deadline); // Store the deadline timestamp
    }

    // Function allowing users to contribute EGLD to the campaign
    #[endpoint] // Marks this as a public endpoint that can be called from outside
    #[payable("EGLD")] // Indicates this function can receive EGLD payments
    fn fund(&self) {
        let payment = self.call_value().egld(); // Get the amount of EGLD sent with the transaction

        require!(
            self.status() == Status::FundingPeriod,
            "cannot fund after deadline"
        ); // Ensure campaign is still in funding period

        let caller = self.blockchain().get_caller(); // Get address of the sender
        self.deposit(&caller)
            .update(|deposit| *deposit += &*payment); // Add payment to caller's deposit record
    }

    // View function to check the current status of the campaign
    #[view]
    fn status(&self) -> Status {
        if self.get_current_time() <= self.deadline().get() {
            Status::FundingPeriod // If deadline hasn't passed, campaign is still active
        } else if self.get_current_funds() >= self.target().get() {
            Status::Successful // If deadline passed and target reached, campaign is successful
        } else {
            Status::Failed // If deadline passed but target not reached, campaign failed
        }
    }

    // View function to get the total amount of funds raised so far
    #[view(getCurrentFunds)] // Specifies the name in the ABI
    fn get_current_funds(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0) // Get contract's EGLD balance
    }

    // Function to either claim funds (owner) or get refunds (contributors)
    #[endpoint]
    fn claim(&self) {
        match self.status() {
            Status::FundingPeriod => sc_panic!("cannot claim before deadline"), // Cannot claim while campaign is active
            Status::Successful => {
                let caller = self.blockchain().get_caller();
                require!(
                    caller == self.blockchain().get_owner_address(),
                    "only owner can claim successful funding"
                ); // Only the contract owner can withdraw funds from successful campaign

                let sc_balance = self.get_current_funds();
                self.send().direct_egld(&caller, &sc_balance); // Transfer all funds to the owner
            }
            Status::Failed => {
                let caller = self.blockchain().get_caller();
                let deposit = self.deposit(&caller).get(); // Get the caller's deposit amount

                if deposit > 0u32 {
                    self.deposit(&caller).clear(); // Clear the deposit record
                    self.send().direct_egld(&caller, &deposit); // Refund the deposit to the contributor
                }
            }
        }
    }

    // Private helper function to get the current blockchain timestamp
    fn get_current_time(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

    // Storage definitions using MultiversX storage mappers

    // Storage for the funding target amount
    #[view(getTarget)]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;

    // Storage for the campaign deadline timestamp
    #[view(getDeadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<u64>;

    // Storage for tracking each donor's contribution amount
    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
