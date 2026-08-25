#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, Vec};

fn setup_multiparty<'a>() -> (Env, AhjoorEscrowContractClient<'a>, Address, Address, u32) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(AhjoorEscrowContract, ());
    let client = AhjoorEscrowContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let arbiter = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_addr = env
        .register_stellar_asset_contract_v2(token_admin.clone())
        .address();
    let token_admin_client = soroban_sdk::token::StellarAssetClient::new(&env, &token_addr);

    client.initialize(&admin);
    client.add_allowed_token(&admin, &token_addr);
    token_admin_client.mint(&buyer, &1_000);

    let deadline = env.ledger().timestamp() + 100_000;
    let escrow_id = client.create_escrow(
        &buyer, &seller, &arbiter, &500, &token_addr, &deadline,
        &None, &Vec::new(&env), &false, &0u32,
    );

    (env, client, admin, buyer, escrow_id)
}

/// get_multiparty_config returns the approvers and threshold exactly as passed
/// to set_multiparty_approval.
#[test]
fn test_get_multiparty_config_matches_set_values() {
    let (env, client, _admin, buyer, escrow_id) = setup_multiparty();

    let approver1 = Address::generate(&env);
    let approver2 = Address::generate(&env);
    let approver3 = Address::generate(&env);
    let mut approvers = Vec::new(&env);
    approvers.push_back(approver1.clone());
    approvers.push_back(approver2.clone());
    approvers.push_back(approver3.clone());

    client.set_multiparty_approval(&buyer, &escrow_id, &approvers, &2u32);

    let (stored_approvers, stored_threshold) = client
        .get_multiparty_config(&escrow_id)
        .expect("config should be set");
    assert_eq!(stored_approvers, approvers);
    assert_eq!(stored_threshold, 2u32);
}

/// get_multiparty_config returns None for an escrow with no multi-party config.
#[test]
fn test_get_multiparty_config_none_when_unconfigured() {
    let (_env, client, _admin, _buyer, escrow_id) = setup_multiparty();
    assert_eq!(client.get_multiparty_config(&escrow_id), None);
}
