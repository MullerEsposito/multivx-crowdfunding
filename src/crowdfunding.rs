#![no_std]

#[allow(unused_imports)]
use multiversx_sc::{derive_imports::*, imports::*};

pub mod crowdfunding_proxy;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Crowdfunding {
    #[init]
    fn constructor(&self, target: BigUint, deadline: u64) {
        require!(target > 0, "Target must be greater than zero");
        self.target().set(&target);

        require!(deadline > self.blockchain().get_block_timestamp(), "Deadline must be in the future");
        self.deadline().set(&deadline);
    }
    
    #[upgrade]
    fn upgrade(&self) {}

    #[view(getTarget)]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;

    #[view(getDeadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<u64>;

    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getCurrentFunds)]
    fn get_current_funds(&self) -> BigUint {
        self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0)
    }

    #[endpoint]
    #[payable("EGLD")]
    fn fund(&self) {
        require!(self.status() == Status::Funding, "Funding is closed");

        let payment = self.call_value().egld();
        let caller = self.blockchain().get_caller();
        self.deposit(&caller).update(|deposit| *deposit += &*payment);
    }

    #[view]
    fn status(&self) -> Status {
        if self.get_current_time() <= self.deadline().get() {
            Status::Funding
        } else if self.get_current_funds() >= self.target().get() {
            Status::Successful
        } else {
            Status::Failed
        }
    }

    #[endpoint]
    fn claim(&self) {
        match self.status() {
            Status::Funding => sc_panic!("Cannot claim before funding"),
            Status::Successful => {
                let caller = self.blockchain().get_caller();
                require!(
                    caller == self.blockchain().get_owner_address(), 
                    "Only owner can claim successfuly funds"
                );

                let sc_balance = self.get_current_funds();
                self.send().direct_egld(&caller, &sc_balance);
            },
            Status::Failed => {
                let caller = self.blockchain().get_caller();
                let deposit = self.deposit(&caller).get();

                if deposit > 0u32 {
                    self.deposit(&caller).clear();
                    self.send().direct_egld(&caller, &deposit);
                }
            },
        }
    }

    fn get_current_time(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Clone, Copy, Debug)]
pub enum Status {
    Funding,
    Successful,
    Failed
}
