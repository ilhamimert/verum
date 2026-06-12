<p align="center">
  <img src="assets/logo.jpg" alt="VERUM logo" width="280">
</p>

# VERUM (VRM)

[![CI](https://github.com/ilhamimert/verum/actions/workflows/ci.yml/badge.svg)](https://github.com/ilhamimert/verum/actions/workflows/ci.yml)

> *"Prove the truth without revealing it."*

🇹🇷 [Türkçe README](README.md)

**VERUM** is an experimental blockchain protocol and ERC-20 token combining
AI-driven consensus (PoAI), zero-knowledge privacy (ZK-Shield), and native
real-world asset anchoring (RealAnchor).

## 🟢 Live — Sepolia Testnet

| | |
|---|---|
| **Token** | VERUM (VRM) — ERC-20 |
| **Contract** | [`0x4b1b5B57504C7269847f2593D77d354Ad72e84C9`](https://eth-sepolia.blockscout.com/token/0x4b1b5B57504C7269847f2593D77d354Ad72e84C9) |
| **Total supply** | 21,000,000 VRM (fixed — no mint function) |
| **Deflation** | 50% of every transfer fee is burned — [proven on-chain](https://eth-sepolia.blockscout.com/tx/0x5448e4747700dad0c2b70aa11287ec2967b8565482ed49d28b9d6489c7497f8c) |
| **Source code** | Verified on explorer ✅ |

## Repository Layout

```
verum/
├── WHITEPAPER.md      # Protocol whitepaper (Turkish, v1.0)
├── WHITEPAPER.en.md   # Protocol whitepaper (English)
├── verum-token/       # ERC-20 contract (Solidity + OpenZeppelin v5)
│   ├── contracts/     #   VerumToken.sol — fixed 21M supply, burn mechanics
│   └── test/          #   14 unit tests (Hardhat)
└── verum-core/        # Standalone L1 chain prototype (Rust)
    └── src/           #   PoAI consensus, P2P network, ZK-Shield, wallet, web UI
```

## verum-token — ERC-20 Contract

A faithful implementation of the whitepaper §9 tokenomics:

- **Fixed supply of 21,000,000 VRM** — minting is impossible after deployment
- **0.1% transfer fee** → half is burned (supply shrinks permanently), half goes to the treasury
- **1% hard fee cap baked into the bytecode** — not even the owner can exceed it
- `Ownable2Step` + `ERC20Permit`, built on OpenZeppelin v5

```bash
cd verum-token
npm install
npm test        # 14 unit tests
```

## verum-core — Rust L1 Prototype

A working prototype of the future standalone VERUM network:

- **PoAI consensus** — simulated committee of 7 AI nodes, 67% majority
- **P2P network** — TCP, deterministic genesis, block propagation & sync
- **ZK-Shield** — commitment/nullifier based shielded transfer simulation
- **Wallet** — secp256k1 + ECDSA, Base58Check `VRM` addresses
- **Web dashboard** — block explorer + wallet panel

```bash
cd verum-core
cargo test                 # unit tests
cargo run                  # 3-node P2P + ZK-Shield demo
cargo run --bin verum-web  # http://127.0.0.1:3000 — web dashboard
```

## Roadmap Status

- [x] Whitepaper v1.0
- [x] Rust core prototype (consensus, P2P, ZK, wallet, web)
- [x] ERC-20 contract + unit tests
- [x] Sepolia testnet deployment + source verification
- [x] Burn mechanism proven on-chain
- [ ] Fuzz testing (Foundry)
- [ ] Independent security audit
- [ ] Mainnet + liquidity

## Disclaimer

This project is **experimental and educational**. The token currently lives
only on the Sepolia test network and has no monetary value. The PoAI and
ZK-Shield components are proof-of-concept simulations. Nothing in this
repository constitutes investment advice.

## License

MIT
