#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String, BytesN, Address};
use soroban_sdk::testutils::{arbitrary::arbitrary, Address as _, Ledger};
use crate::{EnergyTrading, EnergyTradingClient};



// #[test]
// fn test_track_generation() {
//     let env = Env::default();
//     let contract_id = env.register(EnergyTrading, ()); //BytesN::from_array(&env, &[0; 32]);
//     // env.register_contract(&contract_id, EnergyTrading);

//     let client = EnergyTradingClient::new(&env, &contract_id);
//     let user = Address::generate(&env);

//     // Track energy generation
//     client.track_generation(&user, &100);

//     // Verify energy generation
//     env.as_contract(&contract_id, || {
//         let generation_key = soroban_sdk::symbol_short!("gen");
//         let user_generation = env.storage().persistent().get(&(generation_key, &user)).unwrap_or(0);
//         assert_eq!(user_generation, 100);
//     });
// }

#[test]
fn test_track_generation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EnergyTrading); // Register the contract
    let client = EnergyTradingClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    // Track energy generation
    client.track_generation(&user, &100);

    // Verify energy generation
    env.as_contract(&contract_id, || {
        let generation_key = soroban_sdk::symbol_short!("gen");
        let user_generation: i128 = env.storage().persistent().get(&(generation_key, &user)).unwrap_or(0);
        assert_eq!(user_generation, 100);
    });
}

#[test]
fn test_track_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(EnergyTrading {}, ()); 
    // env.register(&contract_id, EnergyTrading);

    let client = EnergyTradingClient::new(&env, &contract_id);
    let from = Address::generate(&env);
    let to = Address::generate(&env);

    // Track energy generation for the sender
    client.track_generation(&from, &100);

    // Transfer energy
    client.track_transfer(&from, &to, &50);

    // Verify energy balances
    env.as_contract(&contract_id, || {
        let transfer_key = soroban_sdk::symbol_short!("transfer");
        let from_balance = env.storage().persistent().get(&(transfer_key.clone(), &from)).unwrap_or(0);
        let to_balance = env.storage().persistent().get(&(transfer_key.clone(), &to)).unwrap_or(0);
        assert_eq!(from_balance, 50);
        assert_eq!(to_balance, 50);
    });
}

#[test]
fn test_track_consumption() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(EnergyTrading {}, ()); // Register the contract
    let client = EnergyTradingClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    // Track energy generation
    client.track_generation(&user, &100);

    // Track energy consumption
    client.track_consumption(&user, &50);

    // Verify energy consumption
    env.as_contract(&contract_id, || {
        let consumption_key = soroban_sdk::symbol_short!("consume");
        let user_consumption: i128 = env.storage().persistent().get(&(consumption_key, &user)).unwrap_or(0);
        assert_eq!(user_consumption, 50);
    });
}

// #[test]
// fn test_track_transfer() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, EnergyTrading); // Register the contract
//     let client = EnergyTradingClient::new(&env, &contract_id);
//     let from = Address::generate(&env);
//     let to = Address::generate(&env);

//     // Track energy generation for the sender
//     client.track_generation(&from, &100);

//     // Transfer energy
//     client.track_transfer(&from, &to, &50);

//     // Verify energy balances
//     env.as_contract(&contract_id, || {
//         let transfer_key = soroban_sdk::symbol_short!("trans");
//         let from_balance: i128 = env.storage().persistent().get(&(transfer_key.clone(), &from)).unwrap_or(0);
//         let to_balance: i128 = env.storage().persistent().get(&(transfer_key.clone(), &to)).unwrap_or(0);
//         assert_eq!(from_balance, 50);
//         assert_eq!(to_balance, 50);
//     });
// }

#[test]
fn test_tokenize_energy() {
    let env = Env::default();
    // env.mock_all_auths();
    let contract_id = env.register_contract(None, EnergyTrading); // Register the contract
    let client = EnergyTradingClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    // Set admin privileges
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&admin, &true);
    });

    // Tokenize energy
    client.tokenize_energy(&admin, &user, &100);

    // Verify tokenized energy
    env.as_contract(&contract_id, || {
        let token_key = soroban_sdk::symbol_short!("etoken");
        let user_tokens: i128 = env.storage().persistent().get(&(token_key, &user)).unwrap_or(0);
        assert_eq!(user_tokens, 100);
    });
}

// #[test]
// fn test_track_consumption() {
//     let env = Env::default();
//     let contract_id = env.register(EnergyTrading, ());
//     // env.register_contract(&contract_id, EnergyTrading);

//     let client = EnergyTradingClient::new(&env, &contract_id);
//     let user = Address::generate(&env);

//     // Track energy generation
//     client.track_generation(&user, &100);

//     // Track energy consumption
//     client.track_consumption(&user, &50);

//     // Verify energy consumption
//     env.as_contract(&contract_id, || {
//         let consumption_key = soroban_sdk::symbol_short!("cons");
//         let user_consumption = env.storage().persistent().get(&(consumption_key, &user)).unwrap_or(0);
//         assert_eq!(user_consumption, 50);
//     });
// }

// #[test]
// fn test_tokenize_energy() {
//     let env = Env::default();
//     let contract_id = env.register(EnergyTrading, ());
//     // env.register_contract(&contract_id, EnergyTrading);

//     let client = EnergyTradingClient::new(&env, &contract_id);
//     let admin = Address::generate(&env);
//     let user = Address::generate(&env);

//     // Set admin privileges
    
//     env.as_contract(&contract_id, || {
//         env.storage().persistent().set(&admin, &true);
//     });

//     // Tokenize energy
//     client.tokenize_energy(&admin, &user, &100);

//     // Verify tokenized energy
//     env.as_contract(&contract_id, || {
//         let token_key = soroban_sdk::symbol_short!("e_tok");
//         let user_tokens = env.storage().persistent().get(&(token_key, &user)).unwrap_or(0);
//         assert_eq!(user_tokens, 100);
//     });
// }
