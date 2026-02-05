# NebulaVRF Contract Guide

This guide shows how to use the NebulaVRF Soroban contract on testnet, including the required formats, DST configuration, and a full commit → reveal → derive workflow.

---

## Contract Information

- **Contract ID**: `CBLC5COYONIRCPWWCRNE6S2EOYJ7IQOWI7RUDG5ZVPHCLZW2KXFFCK2Q`
- **Network**: Soroban Testnet
- **Status**: Deployed

---

## Cryptographic Requirements

### Domain Separation Tag (DST)

All off-chain signing must use the **exact same DST** as the contract:

```
NEBULA-VRF-V01-BLS12381G2
```

### Message Format

The contract verifies signatures over:

```
m = sha256(seed || salt)
```

- `seed` and `salt` are arbitrary bytes chosen off-chain.
- `seed || salt` means raw byte concatenation.
- `m` is always 32 bytes.

### Group Encodings (Uncompressed)

The contract expects **uncompressed** group elements:

- **Public Key (G1)**: 96 bytes
- **Signature (G2)**: 192 bytes

If compressed values are used, verification will fail.

---

## Payload Generation (Testnet)

The helpers in `vrf-core` are **demo utilities** meant to get you started on testnet.
They are not a production randomness strategy. For real integrations, you should
create **your own seed/salt generation and signing flow** tailored to your app.

You can generate correct payloads using the `vrf-core` repo
([NebulaVRF/vrf-core](https://github.com/NebulaVRF/vrf-core)):

```bash
cd vrf-core
cargo run --example sample_payloads
```

This produces hex and base64 outputs for:
- `seed`
- `salt`
- `commitment`
- `pubkey` (G1, 96 bytes)
- `signature` (G2, 192 bytes)

### Build Your Own Seed/Salt Logic

You should replace the demo seed/salt logic with your own method:
- OS randomness (`OsRng`)
- deterministic seed based on app state
- user-provided secrets

At minimum, ensure you:
1. Generate `seed` and `salt` with your own logic.
2. Compute the commitment.
3. Sign with the correct DST and uncompressed formats.

Commitment formula:

```
commitment = sha256(seed || salt)
```

Then sign that commitment using the correct DST.

### Recommended Custom Flow (Summary)

1. **Seed**: derive from your app logic + secure entropy.
2. **Salt**: additional nonce per request.
3. **Commit**: store `sha256(seed || salt)` on-chain.
4. **Reveal**: publish `seed`, `salt`, and signature.
5. **Derive**: use `derive_random` with app‑specific context.

---

## On-Chain Workflow

### 1) Commit

Call `commit` with:
- `user`: your address (must authorize)
- `commitment`: `sha256(seed || salt)`
- `pubkey`: uncompressed G1 (96 bytes)

### 2) Reveal

Call `reveal` with:
- `user`: same address as commit
- `seed`: original seed bytes
- `salt`: original salt bytes
- `signature`: uncompressed G2 (192 bytes)

Return value:
- `BytesN<32>` randomness (same as commitment)

### 3) Derive Random

Call `derive_random` with:
- `randomness`: returned from reveal
- `context`: arbitrary bytes (app-specific)

Returns:
- `u64` deterministic random number

---

## Example Contexts

Use `context` to namespace your randomness:

- `"lottery_round_1"` → `bG90dGVyeV9yb3VuZF8x`
- `"nft_mint_42"` → `bmZ0X21pbnRfNDI=`
- `"dao_vote_7"` → `ZGFvX3ZvdGVfNw==`

---

## Local Payload API (Optional)

The local payload API lives in the `vrf-core` repo
([NebulaVRF/vrf-core](https://github.com/NebulaVRF/vrf-core)):

```bash
cd vrf-core
cargo run --bin nebula_vrf_api --features api
```

Then:
- `GET http://localhost:3000/payloads`
- `GET http://localhost:3000/payloads?seed_len=8&salt_len=8`

Response format:

```
{
  "hex": { ... },
  "base64": { ... }
}
```

---

## Contract Errors

- `1` — `InvalidSignature`
- `2` — `CommitmentMismatch`
- `3` — `NoCommitmentFound`
