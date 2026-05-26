#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_initialize_admin() {

    let env = Env::default();

    let contract_id = env.register_contract(None, ReliefLinkContract);

    let client = ReliefLinkContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize_admin(&admin);
}

#[test]
fn test_allocate_funds() {

    let env = Env::default();

    let contract_id = env.register_contract(None, ReliefLinkContract);

    let client = ReliefLinkContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    let resident = Address::generate(&env);

    client.initialize_admin(&admin);

    client.allocate_funds(
        &admin,
        &resident,
        &1000,
    );

    let balance = client.get_balance(&resident);

    assert_eq!(balance, 1000);
}

#[test]
fn test_claim_relief() {

    let env = Env::default();

    let contract_id = env.register_contract(None, ReliefLinkContract);

    let client = ReliefLinkContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    let resident = Address::generate(&env);

    client.initialize_admin(&admin);

    client.allocate_funds(
        &admin,
        &resident,
        &500,
    );

    client.claim_relief(&resident);

    let status = client.check_claim_status(&resident);

    assert_eq!(status, true);
}
