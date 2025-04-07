use multiversx_sc_snippets::imports::*;
use rust_interact::{config::Config, ContractInteract};

// Simple deploy test that runs on the real blockchain configuration.
// In order for this test to work, make sure that the `config.toml` file contains the real blockchain config (or choose it manually)
// Can be run with `sc-meta test`.
#[tokio::test]
#[ignore = "run on demand, relies on real blockchain state"]
async fn deploy_test_crowdfunding() {
    let mut interactor = ContractInteract::new(Config::new()).await;
    let target: u64 = 100_000_000_000;
    let deadline: u64 = 127_000;

    interactor.deploy(target, deadline).await;
}
