pub mod structs;

use structs::crowdfunding_test_state::CrowdfundingTestState;
use crowdfunding::crowdfunding_proxy::Status;
use multiversx_sc_scenario::imports::*;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const CROWDFUNDING_ADDRESS: TestSCAddress = TestSCAddress::new("crowdfunding");
const DONOR_ONE_ADDRESS: TestAddress = TestAddress::new("donor_one");
const DONOR_TWO_ADDRESS: TestAddress = TestAddress::new("donor_two");
const DONOR_ONE_DONATION: u64 = 100_000_000_000;
const DONOR_TWO_DONATION: u64 = 200_000_000_000;

const TARGET: u64 = 200_000_000_000;
const DEADLINE: u64 = 123_000;
const AFTER_DEADLINE: u64 = 123_001;
const INITIAL_BALANCE: u64 = 300_000_000_000;

#[test]
fn crowdfunding_deploy_test() {
    let mut state = CrowdfundingTestState::new();
    
    state.set_balance(OWNER_ADDRESS, INITIAL_BALANCE);

    state.deploy(OWNER_ADDRESS, TARGET, DEADLINE);

    state.world.check_account(OWNER_ADDRESS).balance(INITIAL_BALANCE);

    state.check_target(TARGET);

    state.check_deadline(DEADLINE);
}

#[test]
fn crowdfunding_fund_test() {
    let mut state = CrowdfundingTestState::new();

    state.set_balance(OWNER_ADDRESS, INITIAL_BALANCE);
    state.deploy(OWNER_ADDRESS, TARGET, DEADLINE);

    state.set_balance(DONOR_ONE_ADDRESS, INITIAL_BALANCE);

    state.world.check_account(OWNER_ADDRESS).nonce(1).balance(INITIAL_BALANCE);
    state.world.check_account(DONOR_ONE_ADDRESS).nonce(0).balance(INITIAL_BALANCE);

    
    state.fund(DONOR_ONE_ADDRESS, DONOR_ONE_DONATION);

    state.world
        .check_account(DONOR_ONE_ADDRESS)
        .nonce(1)
        .balance(INITIAL_BALANCE - DONOR_ONE_DONATION);
    
    state.world
        .check_account(CROWDFUNDING_ADDRESS)
        .nonce(0)
        .balance(DONOR_ONE_DONATION);

    state.check_target(TARGET);

    state.check_deadline(DEADLINE);

    state.check_deposit(DONOR_ONE_ADDRESS, DONOR_ONE_DONATION);
}

#[test]
fn crowdfunding_fund_too_late_test() {
    let mut state = CrowdfundingTestState::new();

    state.set_balance(OWNER_ADDRESS, INITIAL_BALANCE);

    state.deploy(OWNER_ADDRESS, TARGET, DEADLINE);

    state.set_balance(DONOR_ONE_ADDRESS, INITIAL_BALANCE);

    state.world.current_block().block_timestamp(AFTER_DEADLINE);

    state.check_status(Status::Failed);
}

#[test]
fn crowdfunding_claim_test() {
    let mut state = CrowdfundingTestState::new();

    state.set_balance(OWNER_ADDRESS, INITIAL_BALANCE);

    state.deploy(OWNER_ADDRESS, TARGET, DEADLINE);

    state.set_balance(DONOR_ONE_ADDRESS, INITIAL_BALANCE);
    state.set_balance(DONOR_TWO_ADDRESS, INITIAL_BALANCE);

    state.check_status(Status::Funding);

    state.fund(DONOR_ONE_ADDRESS, DONOR_ONE_DONATION);

    state.fund(DONOR_TWO_ADDRESS, DONOR_TWO_DONATION);

    state.world.current_block().block_timestamp(AFTER_DEADLINE);

    state.check_status(Status::Successful);
    
    state.claim_by_not_owner(DONOR_ONE_ADDRESS);

    state.claim_by_owner(OWNER_ADDRESS);
}