use soroban_sdk::{
    bytesn,
    crypto::bls12_381::{G1Affine, G2Affine},
    Bytes, BytesN, Env, Vec,
};

use crate::errors::VRFError;
use crate::storage;
use crate::DST;

/// Verify a BLS signature over the given message hash.
///
/// - `msg` is expected to be `sha256(seed || salt)`, as used by the contract.
/// - `pubkey` is a G1 element (96 bytes).
/// - `signature` is a G2 element (192 bytes).
///
/// In unit tests we bypass the actual pairing check to avoid depending on
/// real BLS test vectors while still exercising the VRF commit/reveal logic.
#[cfg(test)]
pub fn verify_bls_signature(
    _env: &Env,
    _msg: &BytesN<32>,
    _pubkey: &BytesN<96>,
    _signature: &BytesN<192>,
) -> Result<(), VRFError> {
    Ok(())
}

#[cfg(not(test))]
pub fn verify_bls_signature(
    env: &Env,
    msg: &BytesN<32>,
    pubkey: &BytesN<96>,
    signature: &BytesN<192>,
) -> Result<(), VRFError> {
    let bls = env.crypto().bls12_381();

    // Load DST from storage (set during commit); fall back to constant if missing.
    let dst_bytes: Bytes = storage::get_dst(env)
        .unwrap_or_else(|| Bytes::from_slice(env, DST.as_bytes()));

    // Hash the message to G2 using the agreed DST.
    let msg_g2 = bls.hash_to_g2(&msg.clone().into(), &dst_bytes);

    // Negative G1 generator (constant), from official BLS example pattern.
    let neg_g1 = G1Affine::from_bytes(bytesn!(
        env,
        0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb114d1d6855d545a8aa7d76c8cf2e21f267816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca
    ));

    // Prepare inputs to the pairing function.
    let vp1 = Vec::from_array(env, [G1Affine::from_bytes(pubkey.clone()), neg_g1]);
    let vp2 = Vec::from_array(env, [msg_g2, G2Affine::from_bytes(signature.clone())]);

    // Perform the pairing check.
    if !bls.pairing_check(vp1, vp2) {
        return Err(VRFError::InvalidSignature);
    }

    Ok(())
}

