use soroban_sdk::{Address, Bytes, BytesN, Env};

use crate::bls::verify_bls_signature;
use crate::errors::VRFError;
use crate::storage;

/// User commits to a hash and provides their BLS pubkey.
pub fn commit(env: &Env, user: &Address, commitment: BytesN<32>, pubkey: BytesN<96>) {
    user.require_auth();
    storage::set_commitment(env, user, &commitment);
    storage::set_pubkey(env, user, &pubkey);
    storage::set_dst(env, &Bytes::from_slice(env, crate::DST.as_bytes()));
}

/// User reveals their seed, salt, and BLS signature. Contract verifies all.
pub fn reveal(
    env: &Env,
    user: &Address,
    seed: Bytes,
    salt: Bytes,
    signature: BytesN<192>,
) -> Result<BytesN<32>, VRFError> {
    let stored_commitment =
        storage::get_commitment(env, user).ok_or(VRFError::NoCommitmentFound)?;

    // Hash(seed + salt)
    let mut combined = seed.clone();
    combined.append(&salt);
    let recomputed_hash = env.crypto().sha256(&combined);
    let recomputed = BytesN::from_array(env, &recomputed_hash.into());

    if recomputed != stored_commitment {
        return Err(VRFError::CommitmentMismatch);
    }

    let pubkey =
        storage::get_pubkey(env, user).ok_or(VRFError::InvalidSignature)?;

    // Verify BLS signature against the recomputed message.
    verify_bls_signature(env, &recomputed, &pubkey, &signature)?;

    Ok(recomputed)
}

/// Deterministically derive a random u64 from the verified randomness and context.
///
/// The caller provides an arbitrary context as bytes; this is concatenated
/// with the randomness and hashed to derive a u64.
pub fn derive_random(env: Env, randomness: BytesN<32>, context: Bytes) -> u64 {
    let mut extended = Bytes::from_array(&env, &randomness.to_array());
    extended.append(&context);
    let hashed = env.crypto().sha256(&extended);
    let hashed_bytes: [u8; 32] = hashed.into();
    let mut num_bytes = [0u8; 8];
    num_bytes.copy_from_slice(&hashed_bytes[..8]);
    u64::from_le_bytes(num_bytes)
}

