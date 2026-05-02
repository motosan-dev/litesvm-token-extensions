# litesvm-token-extensions

Token-2022 mint and account extension builders for LiteSVM tests.

[![Crates.io](https://img.shields.io/crates/v/litesvm-token-extensions.svg)](https://crates.io/crates/litesvm-token-extensions)
[![Docs](https://docs.rs/litesvm-token-extensions/badge.svg)](https://docs.rs/litesvm-token-extensions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

> **Status:** v0.1 (initial release).

## What this is

Fills the gap in [`litesvm-token`](https://crates.io/crates/litesvm-token) v0.11
where `CreateMint` (even with the `token-2022` feature) only constructs vanilla
Token-2022 mints with no extensions.

Provides two fluent builders mirroring `litesvm-token::CreateMint`'s API style:

- **`CreateMintWithExtensions`** — 7 mint extensions: `TransferFee`, `TransferHook`,
  `MintCloseAuthority`, `MetadataPointer`, `NonTransferable`, `PermanentDelegate`,
  `InterestBearing`
- **`CreateAccountWithExtensions`** — 3 account extensions: `ImmutableOwner`
  (pre-init), `MemoTransfer` (post-init enable), `CpiGuard` (post-init enable)

This crate has NO runtime dependency on `litesvm-token` — the naming reflects
ecosystem positioning, not a runtime relationship.

## Installation

```toml
[dev-dependencies]
litesvm-token-extensions = "0.1"
litesvm = "0.11"
solana-keypair = "3"
solana-pubkey = "4"
solana-signer = "3"
```

MSRV: Rust 1.89.

## Usage

### Mint with extensions

```rust
use litesvm::LiteSVM;
use litesvm_token_extensions::CreateMintWithExtensions;
use solana_keypair::Keypair;
use solana_signer::Signer;

let mut svm = LiteSVM::new();
let payer = Keypair::new();
svm.airdrop(&payer.pubkey(), 100_000_000_000).unwrap();

let mint = CreateMintWithExtensions::new(&mut svm, &payer)
    .decimals(6)
    .with_transfer_fee(250, 5_000_000)
    .with_metadata_pointer(payer.pubkey())
    .send()
    .unwrap();
```

### Account with extensions

```rust
use litesvm_token_extensions::CreateAccountWithExtensions;

let account = CreateAccountWithExtensions::new(&mut svm, &payer, &mint)
    .with_immutable_owner()
    .with_memo_transfer()
    .send()
    .unwrap();
```

### Account with non-payer owner

If you want post-init extensions on an account owned by someone other than the
payer, pass that owner's `&Keypair` to `.owner()` so the builder can sign the
enable instructions:

```rust
let user = Keypair::new();
let account = CreateAccountWithExtensions::new(&mut svm, &payer, &mint)
    .owner(&user)
    .with_cpi_guard()
    .send()
    .unwrap();
```

## Supported extensions

| Extension | Builder method | Phase |
|---|---|---|
| `TransferFeeConfig` | `.with_transfer_fee(bps, max_fee)` | mint pre-init |
| `TransferHook` | `.with_transfer_hook(program_id)` | mint pre-init |
| `MintCloseAuthority` | `.with_mint_close_authority(authority)` | mint pre-init |
| `MetadataPointer` | `.with_metadata_pointer(metadata_address)` | mint pre-init |
| `NonTransferable` | `.with_non_transferable()` | mint pre-init |
| `PermanentDelegate` | `.with_permanent_delegate(delegate)` | mint pre-init |
| `InterestBearingConfig` | `.with_interest_bearing(rate_authority, rate)` | mint pre-init |
| `ImmutableOwner` | `.with_immutable_owner()` | account pre-init |
| `MemoTransfer` | `.with_memo_transfer()` | account post-init enable |
| `CpiGuard` | `.with_cpi_guard()` | account post-init enable |

## v0.2 Roadmap

- `DefaultAccountState` mint extension
- `Group` / `Member` / `GroupMember` extensions
- Mint extension `update_*` instructions

## License

Dual-licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
