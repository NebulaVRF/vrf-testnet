 # NebulaVRF Soroban Contracts

NebulaVRF is a Soroban smart contract that provides verifiable, on-chain randomness using BLS12-381 and a commit–reveal scheme. This repo contains the testnet contract and integration docs; payload generation utilities and the local API live in `vrf-core`.

---

## Contents

- `contracts/nebula-vrf`: Soroban contract implementation
- `USAGE.md`: Testnet guide for using the contract
- `LICENSE`, `CONTRIBUTING.md`: OSS metadata

Related repo:
- `vrf-core`: helper utilities, sample payload generator, and local API ([NebulaVRF/vrf-core](https://github.com/NebulaVRF/vrf-core))

---

## Contract Overview

The contract exposes:
- `commit(user, commitment, pubkey)`
- `reveal(user, seed, salt, signature) -> BytesN<32>`
- `derive_random(randomness, context) -> u64`
- `get_commitment(user) -> Option<BytesN<32>>`
- `get_pubkey(user) -> Option<BytesN<96>>`

Randomness flow:
1. User commits `sha256(seed || salt)` and their BLS G1 public key.
2. User reveals `seed`, `salt`, and a BLS G2 signature over the commitment.
3. Contract verifies and returns the randomness (`BytesN<32>`).
4. Apps derive deterministic randomness with `derive_random`.

NebulaVRF ensures randomness is verifiable, deterministic for auditors, and unbiased through the commit–reveal process.

---

## Cryptographic Standard

The contract expects **uncompressed** points:

- **Public Key (G1)**: 96 bytes
- **Signature (G2)**: 192 bytes

**DST** (must match off-chain signing):

```
NEBULA-VRF-V01-BLS12381G2
```

Message format:

```
m = sha256(seed || salt)
```

---

## Build & Test

From the repo root:

```bash
cd contracts/nebula-vrf
stellar contract build
```

Run tests:

```bash
cd contracts/nebula-vrf
cargo test
```

---

## Contract Deployment

See `DEPLOYMENT.md` for build and deploy instructions (testnet).

Current deployed contract:
- **Contract ID**: `CDNMUKSXRCJNHHKDEND7YKOT62ZEPTEIWL3GVJ7ILOEXUVZYR34FBQCC`
- **Network**: Soroban Testnet

---

## Usage Guide

Full testnet usage is documented in `USAGE.md`.

---

## Payload Generation Helpers

In the `vrf-core` repo:

```bash
cd vrf-core
cargo run --example sample_payloads
```

This generates valid `seed`, `salt`, `commitment`, `pubkey`, `signature` in hex and base64.

---

## Local Payload API

The local payload API lives in the `vrf-core` repo:
([NebulaVRF/vrf-core](https://github.com/NebulaVRF/vrf-core))

```bash
cd vrf-core
cargo run --bin nebula_vrf_api
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

## Repository Layout

```
.
├── contracts/
│   └── nebula-vrf/
│       ├── src/
│       │   ├── bls.rs
│       │   ├── errors.rs
│       │   ├── lib.rs
│       │   ├── storage.rs
│       │   ├── test.rs
│       │   └── vrf.rs
│       ├── Cargo.toml
│       └── README.md
├── Cargo.toml
├── USAGE.md
├── DEPLOYMENT.md
├── LICENSE
└── CONTRIBUTING.md
```

---

## OSS Standards

- `LICENSE` (MIT)
- `CONTRIBUTING.md`

If you want additional OSS files, we can add:
- `SECURITY.md`
- `CHANGELOG.md`
- GitHub issue templates

---

## Support / Contact

Open an issue with:
- steps to reproduce
- expected vs actual behavior
- logs or error output
