# ReliefLink

## Overview

ReliefLink is a blockchain-powered disaster relief distribution system built on Stellar Soroban smart contracts. The platform enables transparent, secure, and tamper-resistant aid allocation and claiming using on-chain records.

## Features

- Allocate disaster aid securely
- Claim relief through blockchain verification
- Prevent duplicate claims
- Transparent and immutable on-chain records
- Smart contract-based disaster relief workflow

## Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Test

```bash
cargo test
```

## Deploy

```bash
stellar contract deploy --wasm target/wasm32-unknown-unknown/release/relieflink2.wasm --source-account default --network testnet --alias relieflinkfinal
```

## Contract Information

### Contract ID

```txt
CBKTBD66A4BAD5NZOJWJSVUI7PYWYIPG2HOMUIZTGZROLGYIPXG2RKBV
```

### Wallet Address

```txt
GAIFQPFWTLTTNWQRBJUMHVUWYCF7F2RGSD6Z5HAMB7ZW6SRS4O4FTXTG
```

## Stellar Testnet Verification

### Wallet Address Explorer
https://stellar.expert/explorer/testnet/account/GAIFQPFWTLTTNWQRBJUMHVUWYCF7F2RGSD6Z5HAMB7ZW6SRS4O4FTXTG

### Smart Contract Explorer
https://stellar.expert/explorer/testnet/contract/CBKTBD66A4BAD5NZOJWJSVUI7PYWYIPG2HOMUIZTGZROLGYIPXG2RKBV

### Contract Deployment Transaction
https://stellar.expert/explorer/testnet/tx/e8e7d2c524089a5aa9224802f3ec45b4beb985560dd70f55b8279f20c5bdea9f

## Smart Contract Functions

### Initialize Admin

```bash
stellar contract invoke --id CBKTBD66A4BAD5NZOJWJSVUI7PYWYIPG2HOMUIZTGZROLGYIPXG2RKBV --source-account default --network testnet -- initialize_admin --admin GAIFQPFWTLTTNWQRBJUMHVUWYCF7F2RGSD6Z5HAMB7ZW6SRS4O4FTXTG
```

### Allocate Funds

```bash
stellar contract invoke --id CBKTBD66A4BAD5NZOJWJSVUI7PYWYIPG2HOMUIZTGZROLGYIPXG2RKBV --source-account default --network testnet -- allocate_funds --admin GAIFQPFWTLTTNWQRBJUMHVUWYCF7F2RGSD6Z5HAMB7ZW6SRS4O4FTXTG --resident GAIFQPFWTLTTNWQRBJUMHVUWYCF7F2RGSD6Z5HAMB7ZW6SRS4O4FTXTG --amount 1000
```

### Claim Relief

```bash
stellar contract invoke --id CBKTBD66A4BAD5NZOJWJSVUI7PYWYIPG2HOMUIZTGZROLGYIPXG2RKBV --source-account default --network testnet -- claim_relief --resident GAIFQPFWTLTTNWQRBJUMHVUWYCF7F2RGSD6Z5HAMB7ZW6SRS4O4FTXTG
```

### Check Claim Status

```bash
stellar contract invoke --id CBKTBD66A4BAD5NZOJWJSVUI7PYWYIPG2HOMUIZTGZROLGYIPXG2RKBV --source-account default --network testnet -- check_claim_status --resident GAIFQPFWTLTTNWQRBJUMHVUWYCF7F2RGSD6Z5HAMB7ZW6SRS4O4FTXTG
```

## Demo Workflow

1. Admin initializes the smart contract
2. Admin allocates disaster aid funds
3. Resident claims allocated relief
4. Smart contract records transactions on-chain
5. Claim status updates transparently on Stellar Testnet

## Technology Stack

- Stellar Soroban Smart Contracts
- Rust Programming Language
- Stellar CLI
- Soroban Studio
- Stellar Testnet

## License

MIT License