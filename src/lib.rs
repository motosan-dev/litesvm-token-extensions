//! litesvm-token-extensions
//!
//! Builders for creating Token-2022 mints and accounts with extensions in LiteSVM tests.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
// Match LiteSVM's native error type in the public API instead of boxing it.
#![allow(clippy::result_large_err)]

pub mod account;
pub mod mint;

mod extensions;

pub use account::CreateAccountWithExtensions;
pub use mint::CreateMintWithExtensions;
