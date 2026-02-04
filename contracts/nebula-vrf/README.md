## NebulaVRF Soroban Contract

NebulaVRF is a Soroban smart contract that exposes a verifiable randomness primitive
on Stellar, built around a commit–reveal scheme and BLS12-381 signatures.

This contract is a thin, on-chain wrapper around the core NebulaVRF design, intended
for use by DeFi, NFT, and DAO applications that need fully on-chain, verifiable
randomness.

---

### Cryptographic model

- **Message**: `m = sha256(seed || salt)` (32-byte hash).
- **Public key**: BLS12-381 `G1` element, serialized as 96 bytes.
- **Signature**: BLS12-381 `G2` element, serialized as 192 bytes.
- **Domain separation tag (DST)**: `NEBULA-VRF-V01-BLS12381G2`.
- On-chain verification uses Soroban's `bls12_381` API and a pairing check based
  on the official BLS signature example from the Stellar docs.

---

### Public interface

All methods are on the `NebulaVRF` contract type.

- **`commit(env, user, commitment, pubkey)`**
  - `user: Address` — the caller's address (must authorize the call).
  - `commitment: BytesN<32>` — `sha256(seed || salt)` chosen by the user.
  - `pubkey: BytesN<96>` — user's BLS public key (G1).
  - Stores `(commitment, pubkey)` for the user in persistent storage and records the DST.

- **`reveal(env, user, seed, salt, signature) -> Result<BytesN<32>, VRFError>`**
  - Recomputes `sha256(seed || salt)` and checks it matches the stored commitment.
  - Loads the stored BLS public key and verifies `signature` against the message and DST.
  - On success, returns the verified `BytesN<32>` randomness (`sha256(seed || salt)`).
  - Errors:
    - `NoCommitmentFound` — no prior `commit` for this `user`.
    - `CommitmentMismatch` — recomputed hash does not match stored commitment.
    - `InvalidSignature` — BLS verification failed (or missing pubkey).

- **`derive_random(env, randomness, context) -> u64`**
  - `randomness: BytesN<32>` — typically the output of `reveal`.
  - `context: Bytes` — arbitrary application-specific context (e.g. `"lottery_round_1"`).
  - Returns a deterministic `u64` derived from `sha256(randomness || context)`.

- **`get_commitment(env, user) -> Option<BytesN<32>>`**
  - Returns the stored commitment for `user`, if any.

- **`get_pubkey(env, user) -> Option<BytesN<96>>`**
  - Returns the stored BLS public key for `user`, if any.

---

### Testing notes

- Unit tests use `env.mock_all_auths()` to bypass `require_auth` during testing.
- In tests, the BLS pairing check is stubbed out under `cfg(test)` to avoid the need
  for real on-curve BLS test vectors; this lets tests focus on storage and commit–reveal
  logic.
- The testnet contract (compiled to WASM for deployment) runs full BLS verification.

