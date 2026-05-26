<img width="1780" height="980" alt="image" src="https://github.com/user-attachments/assets/adbb7db5-d716-40a1-90b1-ed43912ee637" />


# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.

# YenFlow

Cross-border commission escrow for Japanese artists receiving payments from overseas clients using Stellar and Soroban.

---

## Overview

YenFlow is a payment and escrow application designed for independent artists in Japan who earn through international commissions. It enables creators to receive fast and low-cost payments from overseas buyers using Stellar USDC while protecting both parties through a Soroban smart contract escrow system.

The goal is to remove payment delays, reduce fees, and create a safer commission workflow.

---

## Problem

Independent anime illustrators, VTuber asset creators, and digital artists in Japan often receive international commissions through Discord, X, and portfolio websites.

Current payment methods create several issues:

- High PayPal and international transfer fees
- Currency conversion losses
- Delayed payouts (3–7 days)
- Payment disputes and chargeback risks
- Small commissions becoming less profitable

For creators earning through frequent low-value commissions, these costs significantly reduce income.

---

## Solution

YenFlow allows overseas customers to pay using Stellar USDC while funds are temporarily secured inside a Soroban escrow smart contract.

Workflow:

1. Artist creates commission request
2. Customer submits payment
3. Soroban contract locks funds
4. Artist completes commission
5. Buyer confirms delivery
6. Payment automatically releases

This creates trust without relying on traditional payment intermediaries.

---

## Key Features

### Global Commission Payment Links
Artists generate shareable payment links for commissions.

Example:

Illustration commission → 25 USDC payment request

---

### Smart Contract Escrow

Funds remain locked until commission completion is verified.

Flow:

Customer Pays → Escrow Locks → Delivery → Confirmation → Release

---

### Instant International Payments

Uses Stellar USDC for fast settlement.

Benefits:

- Near-instant payments
- Low transaction fees
- No waiting several business days

---

### Delivery Confirmation System

The contract tracks:

- Pending
- Delivered
- Released

This gives both parties transparency.

---

### Artist Reputation System (Future Feature)

Creators may earn on-chain achievement assets:

- Verified Artist
- Top Creator
- 100 Completed Orders

---

### Mobile-First Experience

Many Japanese creators work primarily through mobile apps and social platforms.

YenFlow is designed for mobile use.

---

## Target Users

### Primary Users

Independent:

- Anime illustrators
- Doujin artists
- VTuber creators
- Freelance digital artists

Location:

- Tokyo
- Osaka
- Kyoto
- Japan creator communities

Income range:

¥50,000–¥300,000/month

---

## Stellar Features Used

- USDC transfers
- XLM transaction fees
- Soroban smart contracts
- Trustlines

---

## MVP Demo Flow

Demo duration: under 2 minutes

### Step 1
Artist creates a commission request

### Step 2
Customer sends payment

### Step 3
Funds become escrowed on-chain

### Step 4
Artist marks completed work

### Step 5
Customer confirms

### Step 6
Contract releases payment

---

## Smart Contract Architecture

Main contract functions:

### create_payment()

Creates escrow record.

Inputs:

- customer
- artist
- amount
- commission ID

---

### mark_delivered()

Allows artist to mark work complete.

---

### release()

Allows customer approval and payment release.

---

### get()

Retrieves escrow data.

---

## Project Structure

```text
yen_flow/
│
├── src/
│   ├── lib.rs
│   └── test.rs
│
├── Cargo.toml
│
├── README.md
│
└── frontend/
```

---

## Timeline

### Week 1

- Soroban contract development
- Escrow logic
- Unit testing

### Week 2

- Frontend integration
- Wallet connection

### Week 3

- Testnet deployment
- Demo preparation

---

## Prerequisites

Install Rust:

```bash
https://rustup.rs
```

Install Soroban CLI:

```bash
cargo install soroban-cli
```

Verify installation:

```bash
soroban --version
```

---

## Build

```bash
soroban contract build
```

---

## Test

```bash
cargo test
```

---

## Deploy To Stellar Testnet

```bash
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/yen_flow.wasm \
--source alice \
--network testnet
```

---

## Example Contract Invocation

Create commission:

```bash
soroban contract invoke \
--id CONTRACT_ID \
--source alice \
-- create_payment \
--id 1 \
--customer GABC123 \
--artist GXYZ123 \
--amount 25
```

Mark delivered:

```bash
soroban contract invoke \
--id CONTRACT_ID \
--source artist \
-- mark_delivered \
--id 1 \
--artist GXYZ123
```

Release payment:

```bash
soroban contract invoke \
--id CONTRACT_ID \
--source customer \
-- release \
--id 1 \
--customer GABC123
```

---

## Future Roadmap

### AI Translation Assistant

Automatically translate commission requests between Japanese and English.

---

### Anchor Integration

Allow USDC conversion into local banking systems.

---

### Social Wallet Login

Reduce onboarding friction for non-crypto users.

---

### On-chain Creator Reputation

Portable trust and commission history.

---

## Vision

YenFlow aims to become payment infrastructure for creator economies in Japan by helping artists receive international income instantly and securely.

---

## License

MIT License
Funds protected until work is delivered
Transparent payment status
Trust without intermediaries
