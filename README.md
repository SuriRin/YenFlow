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

# Overview

YenFlow is a cross-border payment and escrow application designed for independent artists in Japan who receive commission work from overseas clients. The project enables artists to accept international payments using Stellar USDC while protecting both buyers and creators through Soroban smart contract escrow.

The primary goal is to remove common problems in international creator payments: high fees, payment delays, currency conversion losses, and disputes.


# YenFlow — Key Features
1. Global Commission Payment Links

Artists generate a shareable payment link for each commission request (example: anime illustration for 25 USDC). Fans from any country can pay instantly.

Benefit: Removes bank transfers and PayPal friction.

2. Soroban Escrow Protection

Payments are locked in a Soroban smart contract until the commission is delivered and confirmed.

Flow:
Customer pays → funds locked → artist delivers → customer confirms → funds released

Benefit: Reduces scam risk for both parties.

3. Instant USDC Settlement

Payments use Stellar USDC for near real-time transfers.

Benefit: Artists avoid waiting several business days for international payments.

4. Low-Cost Cross-Border Payments

Stellar transaction fees are extremely small, making small commissions practical.

Example:
$10 profile icons or $15 VTuber emotes remain profitable.

Benefit: Artists keep more of their earnings.

5. Multi-Currency Friendly Payments

Overseas fans can pay with USDC while artists can later convert funds through Stellar infrastructure.

Benefit: Reduces conversion losses from traditional payment processors.

6. Delivery Confirmation System

Artists mark work as completed, and buyers approve delivery before payment release.

On-chain state changes:

Pending
Delivered
Released

Benefit: Transparent transaction status.

7. Artist Reputation Badge Tokens

Issue non-transferable Stellar custom assets as achievement or trust badges.

Examples:

100 Completed Orders
Verified Artist
Top Creator

Benefit: Creates portable on-chain reputation.

8. Wallet-Light Onboarding

Overseas fans can pay through temporary payment links and social login instead of manually creating wallets.

Benefit: Less crypto knowledge required.

9. Commission Dashboard

Artists view:

Active commissions
Payment status
Escrow progress
Earnings history
Completed orders

Benefit: Simple creator workflow.

10. Mobile-First Creator Experience

Many Japanese independent artists primarily use mobile apps and social platforms like X and Discord.

Benefit: Entire payment flow works from a phone.

