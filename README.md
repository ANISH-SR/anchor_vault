# Anchor Vault

A secure, on-chain vault program for the Solana blockchain built with Anchor framework. This program enables users to create personal vaults, deposit and withdraw funds, and close vaults with full control over their assets.

## Overview

Anchor Vault is a Solana program that implements a secure vault system using Program Derived Addresses (PDAs) for account management. Users can:

- **Initialize** a personal vault with a unique PDA
- **Deposit** SOL into their vault
- **Withdraw** SOL from their vault
- **Close** their vault and recover all funds

The program leverages Solana's native system program through Cross-Program Invocation (CPI) to handle all transfers securely and efficiently.

## Features

- **PDA-Based Security**: Uses Program Derived Addresses for secure, deterministic vault account generation
- **Secure Fund Management**: All transfers are handled through Solana's system program CPI
- **Owner-Controlled Vaults**: Each user has their own vault with complete control
- **Rent-Exempt Accounts**: Vault state accounts are properly initialized with rent-exempt minimum balances
- **Comprehensive Testing**: Full test suite using Mollusk for instruction-level testing
- **Compute Unit Benchmarking**: Built-in benchmarking to track and optimize compute usage
- **Anchor Framework**: Built with Anchor 0.31.1 for type-safe Solana development

## Installation

### Prerequisites

- **Rust** (latest stable version)
- **Solana CLI** (v1.18.0 or later)
- **Anchor CLI** (v0.31.1 or later)
- **Node.js** (v16 or later) and **Yarn**
- **Mollusk** (for testing)

### Setup

1. Clone the repository:

```bash
git clone https://github.com/ANISH-SR/anchor-vault.git
cd anchor-vault
```

2. Install Anchor dependencies:

```bash
anchor build
```

3. Install Node.js dependencies:

```bash
yarn install
```

4. Set up your Solana wallet:

```bash
solana-keygen new
```

5. Configure your Anchor environment:

Update `Anchor.toml` with your preferred cluster (localnet, devnet, or mainnet):

```toml
[provider]
cluster = "localnet"  # or "devnet" / "mainnet-beta"
wallet = "~/.config/solana/id.json"
```

## Usage

### Building the Program

```bash
anchor build
```

### Running Tests

```bash
# Run Anchor tests
anchor test

# Run Rust unit tests with Mollusk
cargo test-sbf
```

### Deploying to Devnet

```bash
# Build and deploy
anchor build
anchor deploy

# Or use the migration script
anchor run migrate
```

### Program Instructions

#### Initialize Vault

Creates a new vault for the user with PDAs for both the vault state and vault account.

```rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()>
```

**Accounts:**
- `user`: Signer account (the vault owner)
- `vault_state`: PDA account storing vault metadata
- `vault`: PDA account holding the SOL
- `system_program`: Solana system program

#### Deposit

Deposits SOL into the user's vault.

```rust
pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()>
```

**Accounts:**
- `user`: Signer account
- `vault_state`: PDA account for vault state
- `vault`: PDA account holding SOL
- `system_program`: Solana system program

**Parameters:**
- `amount`: Amount in lamports to deposit

#### Withdraw

Withdraws SOL from the user's vault back to their wallet.

```rust
pub fn withdraw(ctx: Context<VaultAction>, amount: u64) -> Result<()>
```

**Accounts:**
- `user`: Signer account
- `vault_state`: PDA account for vault state
- `vault`: PDA account holding SOL (signed by PDA)
- `system_program`: Solana system program

**Parameters:**
- `amount`: Amount in lamports to withdraw

#### Close Vault

Closes the vault and transfers all remaining funds back to the user. The vault state account is closed and rent is refunded.

```rust
pub fn close(ctx: Context<Close>) -> Result<()>
```

**Accounts:**
- `user`: Signer account
- `vault_state`: PDA account for vault state (will be closed)
- `vault`: PDA account holding SOL (signed by PDA)
- `system_program`: Solana system program

## Project Structure

```
anchor-vault/
├── programs/
│   └── anchor_vault/
│       ├── src/
│       │   └── lib.rs              # Main program logic
│       ├── tests/
│       │   └── test_vault.rs       # Mollusk-based tests
│       ├── benches/
│       │   └── compute_units.md    # Compute unit benchmarks
│       └── Cargo.toml              # Rust dependencies
├── tests/
│   └── test_vault.rs               # Anchor integration tests
├── migrations/
│   └── deploy.ts                   # Deployment script
├── Anchor.toml                     # Anchor configuration
├── Cargo.toml                      # Workspace configuration
├── package.json                    # Node.js dependencies
└── README.md                       # This file
```

## Architecture

### Account Structure

- **VaultState**: Stores vault metadata including PDA bumps
  - `vault_bump: u8` - Bump seed for the vault PDA
  - `state_bump: u8` - Bump seed for the state PDA

### PDA Derivation

- **State PDA**: `["state", user.key()]`
- **Vault PDA**: `["vault", state_pda.key()]`

### Security Features

- All vault operations require the owner to sign
- Withdrawals and closes use PDA signing for secure transfers
- Account validation through Anchor's account constraints
- Rent-exempt account initialization

## Testing

The project includes comprehensive tests using both Anchor's test framework and Mollusk for low-level instruction testing:

- **Initialize Test**: Verifies vault initialization and rent-exempt account setup
- **Deposit Test**: Tests SOL deposits with balance verification
- **Withdraw Test**: Tests SOL withdrawals with PDA signing
- **Close Test**: Tests vault closure and fund recovery

### Running Tests

```bash
# Anchor tests
anchor test

# Mollusk tests with benchmarking
cargo test-sbf
```

Benchmark results are saved to `programs/anchor_vault/benches/compute_units.md`.

## Error Handling

The program defines custom error codes:

- `VaultAlreadyExists`: Attempted to initialize an existing vault
- `InvalidAmount`: Invalid deposit or withdrawal amount

## Deployment

### Local Development

```bash
# Start local validator
solana-test-validator

# Build and deploy
anchor build
anchor deploy
```

### Devnet Deployment

```bash
# Set cluster to devnet
solana config set --url devnet

# Airdrop SOL (if needed)
solana airdrop 2

# Deploy
anchor build
anchor deploy
```

## Program ID

The program ID is: `FGxVc2HAfo2bDARNMtDRzKwCbRCT8XpvBiYUYqPjLhqt`

## Dependencies

### Rust Dependencies
- `anchor-lang = "0.31.1"` - Anchor framework

### Dev Dependencies
- `mollusk-svm = "0.4.1"` - Solana VM for testing
- `mollusk-svm-bencher = "0.4.1"` - Compute unit benchmarking
- `solana-program = "2.3.0"` - Solana program SDK
- `solana-sdk = "2.3.0"` - Solana SDK

### Node.js Dependencies
- `@coral-xyz/anchor = "^0.31.1"` - Anchor TypeScript client

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- **Anchor Framework** for the excellent Solana development framework
- **Solana Foundation** for the Solana blockchain
- **Mollusk** for the powerful testing and benchmarking tools

