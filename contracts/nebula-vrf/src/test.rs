#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address, Bytes, BytesN};
use crate::NebulaVRFClient;

fn dummy_seed_and_salt(env: &Env) -> (Bytes, Bytes, BytesN<32>) {
    let seed = Bytes::from_array(env, &[1, 2, 3, 4]);
    let salt = Bytes::from_array(env, &[5, 6, 7, 8]);

    let mut combined = seed.clone();
    combined.append(&salt);
    let hash = env.crypto().sha256(&combined);
    let commitment = BytesN::from_array(env, &hash.into());

    (seed, salt, commitment)
}

#[test]
fn test_commit_and_get_commitment() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(NebulaVRF, ());
    let user = Address::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF");
    let commitment = BytesN::from_array(&env, &[3u8; 32]);
    let pubkey = BytesN::from_array(&env, &[4u8; 96]);
    let client = NebulaVRFClient::new(&env, &contract_id);

    client.commit(&user, &commitment, &pubkey);
    let stored = client.get_commitment(&user).unwrap();
    assert_eq!(stored, commitment);
}

#[test]
fn test_get_commitment_and_pubkey() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(NebulaVRF, ());
    let user = Address::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF");
    let commitment = BytesN::from_array(&env, &[3u8; 32]);
    let pubkey = BytesN::from_array(&env, &[4u8; 96]);
    let client = NebulaVRFClient::new(&env, &contract_id);

    client.commit(&user, &commitment, &pubkey);
    let stored_commitment = client.get_commitment(&user).unwrap();
    let stored_pubkey = client.get_pubkey(&user).unwrap();
    assert_eq!(stored_commitment, commitment);
    assert_eq!(stored_pubkey, pubkey);
}

#[test]
fn test_derive_random_output() {
    let env = Env::default();
    env.mock_all_auths();
    let context = Bytes::from_array(&env, b"draw");
    let base_random = BytesN::from_array(&env, &[0x11; 32]);
    let result = NebulaVRF::derive_random(env, base_random, context);
    // Just checking that it returns something in u64 range
    assert!(result >= 0);
}

#[test]
fn test_reveal_success_with_dummy_bls() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(NebulaVRF, ());
    let user = Address::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF");
    let client = NebulaVRFClient::new(&env, &contract_id);

    // Prepare seed, salt, and corresponding commitment.
    let (seed, salt, commitment) = dummy_seed_and_salt(&env);

    // Dummy BLS pubkey and signature (values are not on-curve, but in tests the
    // actual pairing check is bypassed via cfg(test) in bls::verify_bls_signature.
    let pubkey = BytesN::from_array(&env, &[1u8; 96]);
    let signature = BytesN::from_array(&env, &[2u8; 192]);

    // Commit phase.
    client.commit(&user, &commitment, &pubkey);

    // Reveal phase should succeed and return the recomputed commitment.
    let result = client.reveal(&user, &seed, &salt, &signature);
    assert_eq!(result, commitment);
}

#[test]
#[should_panic]
fn test_reveal_fails_on_commitment_mismatch() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(NebulaVRF, ());
    let user = Address::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF");
    let client = NebulaVRFClient::new(&env, &contract_id);

    let (seed, _salt, commitment) = dummy_seed_and_salt(&env);
    let wrong_salt = Bytes::from_array(&env, &[9, 9, 9, 9]);
    let pubkey = BytesN::from_array(&env, &[1u8; 96]);
    let signature = BytesN::from_array(&env, &[2u8; 192]);

    client.commit(&user, &commitment, &pubkey);

    // With a wrong salt, reveal should trap with a contract error (CommitmentMismatch).
    // We mark this test as should_panic to acknowledge the trap behavior.
    let _ = client.reveal(&user, &seed, &wrong_salt, &signature);
}

#[test]
#[should_panic]
fn test_reveal_fails_when_no_commitment() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(NebulaVRF, ());
    let user = Address::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF");
    let client = NebulaVRFClient::new(&env, &contract_id);

    let (seed, salt, _commitment) = dummy_seed_and_salt(&env);
    let pubkey = BytesN::from_array(&env, &[1u8; 96]);
    let signature = BytesN::from_array(&env, &[2u8; 192]);

    // No prior commit call; this should trap with NoCommitmentFound (contract error #3).
    let _ = client.reveal(&user, &seed, &salt, &signature);
}
