## Contributing

Thanks for your interest in contributing to NebulaVRF.

### Development setup

1. Install Rust and the Soroban toolchain.
2. Clone the repository and build the contract:

```
cd vrf-testnet/contracts/nebula-vrf
stellar contract build
```

### Tests

```
cd vrf-testnet/contracts/nebula-vrf
cargo test
```

### Coding standards

- Keep modules small and focused.
- Avoid non-deterministic logic in the contract.
- Use the existing `helpers` and examples for payload generation.
- Prefer clear, minimal APIs over clever abstractions.

### Pull requests

- Describe the problem and the proposed fix.
- Include tests for behavioral changes.
- Update `USAGE.md` when the public API changes.

### Reporting issues

Please include:
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs or error messages
