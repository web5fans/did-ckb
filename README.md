# `did-ckb` Project

This is an on-chain script implementation of [did:ckb Method and did:ckb Method Local ID Extension](https://github.com/web5fans/web5-wips).

## Overview

The `did-ckb` project provides a decentralized identity (DID) solution on the Nervos CKB blockchain. It implements the did:ckb method specification, enabling creation and management of decentralized identifiers with cryptographic verification capabilities on-chain.

## Project Structure

```
did-ckb/
├── contracts/          # Smart contract implementations
│   └── did-ckb-ts/    # Main TypeScript contract
├── crates/            # Rust libraries
│   └── ckb-did-plc-utils/  # Utility functions for DID operations
├── tests/             # Rust integration tests
├── ts-tests/          # TypeScript tests
└── tools/             # Development tools and utilities
```

## Requirements

### Build
- Rust (>= 1.85.1)

### Test
- pnpm
- [ckb-debugger](https://github.com/nervosnetwork/ckb-standalone-debugger)

## Build

```bash
make build
```

## Test

```bash
pnpm install
# Generate test vectors
cd tools/gen-test-vectors && pnpm build && pnpm start
cargo test -p ckb-did-plc-utils-tests
pnpm test
```

## Usage

To integrate the did-ckb contract into your CKB application, reference the deployment parameters below.

## Deployment

### Mainnet

| Parameter   | Value                                                                |
| ----------- | -------------------------------------------------------------------- |
| `code_hash` | `0x4a06164dc34dccade5afe3e847a97b6db743e79f5477fa3295acf02849c5984a` |
| `hash_type` | `type`                                                               |
| `tx_hash`   | `0xe2f74c56cdc610d2b9fe898a96a80118845f5278605d7f9ad535dad69ae015bf` |
| `index`     | `0x0`                                                                |
| `dep_type`  | `code`                                                               |

### Testnet

| Parameter   | Value                                                                |
| ----------- | -------------------------------------------------------------------- |
| `code_hash` | `0x510150477b10d6ab551a509b71265f3164e9fd4137fcb5a4322f49f03092c7c5` |
| `hash_type` | `type`                                                               |
| `tx_hash`   | `0x0e7a830e2d5ebd05cd45a55f93f94559edea0ef1237b7233f49f7facfb3d6a6c` |
| `index`     | `0x0`                                                                |
| `dep_type`  | `code`                                                               |

## Contributing

Contributions are welcome! Please ensure all tests pass before submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Resources

- [did:ckb Method Specification](https://github.com/web5fans/web5-wips)
- [CKB Script Templates](https://github.com/cryptape/ckb-script-templates)


