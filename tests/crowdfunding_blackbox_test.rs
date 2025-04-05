use crowdfunding::crowdfunding_proxy::{self, Status};
use multiversx_sc_scenario::imports::*;

const CODE_PATH: MxscPath = MxscPath::new("output/crowdfunding.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("crowdfunding");
    blockchain.register_contract(CODE_PATH, crowdfunding::ContractBuilder);
    blockchain
}

const OWNER: TestAddress = TestAddress::new("owner");
const CROWDFUNDING_ADDRESS: TestSCAddress = TestSCAddress::new("crowdfunding");
const DONOR: TestAddress = TestAddress::new("donor");

const TARGET: u64 = 200_000_000_000;
const DEADLINE: u64 = 123_000;
const AFTER_DEADLINE: u64 = 123_001;
const DONOR_INITIAL_BALANCE: u64 = 500_000_000_000;
const DONOR_INITIAL_DEPOSIT: u64 = 150_000_000_000;

fn crowdfunding_deploy() -> ScenarioWorld {
    let mut world = world();
    
    world.account(OWNER).nonce(0).balance(1000000);    
    
    let crowdfunding_address = world
        .tx()
        .from(OWNER)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .constructor(TARGET, DEADLINE)
        .code(CODE_PATH)
        .new_address(CROWDFUNDING_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(crowdfunding_address, CROWDFUNDING_ADDRESS.to_address());

    world
}

#[test]
fn crowdfunding_deploy_test() {
    let mut world: ScenarioWorld = crowdfunding_deploy();

    world.check_account(OWNER).balance(1_000_000);

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .target()
        .returns(ExpectValue(TARGET))
        .run();

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .deadline()
        .returns(ExpectValue(DEADLINE))
        .run();
}


fn crowdfunding_fund() -> ScenarioWorld {
    let mut world: ScenarioWorld = crowdfunding_deploy();

    world.account(DONOR).nonce(0).balance(DONOR_INITIAL_BALANCE);

    world
        .tx()
        .from(DONOR)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .fund()
        .egld(DONOR_INITIAL_DEPOSIT)
        .run();

    world
}

#[test]
fn crowdfunding_fund_test() {
    let mut world: ScenarioWorld = crowdfunding_fund();

    world.check_account(OWNER).nonce(1).balance(1_000_000u64);
    world.check_account(DONOR).nonce(1).balance(DONOR_INITIAL_BALANCE - DONOR_INITIAL_DEPOSIT);
    world.check_account(CROWDFUNDING_ADDRESS).nonce(0).balance(DONOR_INITIAL_DEPOSIT);

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .target()
        .returns(ExpectValue(TARGET))
        .run();

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .deadline()
        .returns(ExpectValue(DEADLINE))
        .run();

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .deposit(DONOR)
        .returns(ExpectValue(DONOR_INITIAL_DEPOSIT))
        .run();
}

#[test]
fn crowdfunding_fund_too_late_test() {
    let mut world: ScenarioWorld = crowdfunding_fund();

    world.current_block().block_timestamp(AFTER_DEADLINE);

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .status()
        .returns(ExpectValue(Status::Failed))
        .run();
}

#[test]
fn crowdfunding_claim_test() {
    let mut world: ScenarioWorld = crowdfunding_fund();

    world
        .tx()
        .from(DONOR)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .fund()
        .egld(100_000_000_000u64)
        .run();

    world.current_block().block_timestamp(AFTER_DEADLINE);

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .status()
        .returns(ExpectValue(Status::Successful))
        .run();

    // world
    //     .tx()
    //     .from(DONOR)
    //     .to(CROWDFUNDING_ADDRESS)
    //     .typed(crowdfunding_proxy::CrowdfundingProxy)
    //     .claim()
    //     .with_result(ExpectError(4, "Only owner can claim successfuly funds"))
    //     .run();

}