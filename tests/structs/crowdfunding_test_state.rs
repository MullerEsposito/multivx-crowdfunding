use multiversx_sc_scenario::imports::*;
use crowdfunding::crowdfunding_proxy;

const CODE_PATH: MxscPath = MxscPath::new("output/crowdfunding.mxsc.json");
const CROWDFUNDING_ADDRESS: TestSCAddress = TestSCAddress::new("crowdfunding");

pub struct CrowdfundingTestState {
    pub world: ScenarioWorld,
}

impl CrowdfundingTestState {
    pub fn new() -> Self {
        let world = world();

        Self { world }
    }

    pub fn deploy(&mut self, deployer: TestAddress, target: u64, deadline: u64) {
        self.world
            .tx()
            .from(deployer)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .constructor(target, deadline)
            .code(CODE_PATH)
            .new_address(CROWDFUNDING_ADDRESS)
            .run();
    }

    pub fn fund(&mut self, funder: TestAddress, amount: u64) {
        self.world
            .tx()
            .from(funder)
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .fund()
            .egld(amount)
            .run();
    }

    pub fn claim_by_owner(&mut self, owner: TestAddress) {
        self.world
            .tx()
            .from(owner)
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .claim()
            .run();
    }

    pub fn claim_by_not_owner(&mut self, claimer: TestAddress) {
        self.world
            .tx()
            .from(claimer)
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .claim()
            .with_result(ExpectError(4, "Only owner can claim successfuly funds"))
            .run();
    }

    pub fn check_target(&mut self, target: u64) {
        self.world
            .query()
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .target()
            .returns(ExpectValue(target))
            .run();
    }

    pub fn check_deadline(&mut self, deadline: u64) {
        self.world
            .query()
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .deadline()
            .returns(ExpectValue(deadline))
            .run();
    }

    pub fn check_deposit(&mut self, donor: TestAddress, amount: u64) {
        self.world
            .query()
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .deposit(donor)
            .returns(ExpectValue(amount))
            .run();
    }

    pub fn check_status(&mut self, expected_status: crowdfunding_proxy::Status) {
        self.world
            .query()
            .to(CROWDFUNDING_ADDRESS)
            .typed(crowdfunding_proxy::CrowdfundingProxy)
            .status()
            .returns(ExpectValue(expected_status))
            .run();
    }

    pub fn set_balance(&mut self, address: TestAddress, balance: u64) {
        self.world.account(address).balance(balance);
    }
}


fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("multivx-crowdfunding");
    blockchain.register_contract(CODE_PATH, crowdfunding::ContractBuilder);

    blockchain
}