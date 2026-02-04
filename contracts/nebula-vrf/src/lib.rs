#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Bytes, BytesN, Env};

pub mod bls;
pub mod errors;
pub mod storage;
pub mod vrf;

pub use errors::VRFError;
pub use storage::DataKey;

/// Domain separation tag used for hashing the VRF message onto G2.
///
/// This must be kept in sync with any off-chain signing implementation
/// (e.g., the `nebula-vrf` core crate) so that the on-chain verification
/// and off-chain signatures agree on the exact message and DST.
///
/// Message format:
///   m = sha256(seed || salt)
///
/// The BLS public key is an element of G1 (96 bytes) and the signature
/// is an element of G2 (192 bytes), following the min-signature mode.
pub const DST: &str = "NEBULA-VRF-V01-BLS12381G2";

#[contract]
pub struct NebulaVRF;

#[contractimpl]
impl NebulaVRF {
    /// User commits to a hash and provides their BLS pubkey
    pub fn commit(env: Env, user: Address, commitment: BytesN<32>, pubkey: BytesN<96>) {
        vrf::commit(&env, &user, commitment, pubkey);
    }

    /// User reveals their seed, salt, and BLS signature. Contract verifies all.
    pub fn reveal(
        env: Env,
        user: Address,
        seed: Bytes,
        salt: Bytes,
        signature: BytesN<192>,
    ) -> Result<BytesN<32>, VRFError> {
        vrf::reveal(&env, &user, seed, salt, signature)
    }

    /// Deterministically derive a random u64 from the verified randomness and context.
    pub fn derive_random(env: Env, randomness: BytesN<32>, context: Bytes) -> u64 {
        vrf::derive_random(env, randomness, context)
    }

    /// Utility: returns the stored commitment for a user
    pub fn get_commitment(env: Env, user: Address) -> Option<BytesN<32>> {
        storage::get_commitment(&env, &user)
    }

    /// Utility: returns the stored pubkey for a user
    pub fn get_pubkey(env: Env, user: Address) -> Option<BytesN<96>> {
        storage::get_pubkey(&env, &user)
    }
}

mod test;