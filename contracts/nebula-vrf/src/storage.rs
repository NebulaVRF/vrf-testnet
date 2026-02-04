use soroban_sdk::{contracttype, Address, Bytes, BytesN, Env};

/// Storage keys used by the NebulaVRF contract.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Commitment(Address),
    PubKey(Address),
    Dst,
}

pub fn set_commitment(env: &Env, user: &Address, commitment: &BytesN<32>) {
    env.storage()
        .persistent()
        .set(&DataKey::Commitment(user.clone()), commitment);
}

pub fn get_commitment(env: &Env, user: &Address) -> Option<BytesN<32>> {
    env.storage()
        .persistent()
        .get(&DataKey::Commitment(user.clone()))
}

pub fn set_pubkey(env: &Env, user: &Address, pubkey: &BytesN<96>) {
    env.storage()
        .persistent()
        .set(&DataKey::PubKey(user.clone()), pubkey);
}

pub fn get_pubkey(env: &Env, user: &Address) -> Option<BytesN<96>> {
    env.storage()
        .persistent()
        .get(&DataKey::PubKey(user.clone()))
}

pub fn set_dst(env: &Env, dst: &Bytes) {
    env.storage().instance().set(&DataKey::Dst, dst);
}

pub fn get_dst(env: &Env) -> Option<Bytes> {
    env.storage().instance().get(&DataKey::Dst)
}

