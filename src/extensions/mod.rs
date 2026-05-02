//! Per-extension instruction builders + `ExtensionType` constants.

use solana_pubkey::Pubkey;

pub(crate) mod cpi_guard;
pub(crate) mod immutable_owner;
pub(crate) mod interest_bearing;
pub(crate) mod memo_transfer;
pub(crate) mod metadata_pointer;
pub(crate) mod mint_close_authority;
pub(crate) mod non_transferable;
pub(crate) mod permanent_delegate;
pub(crate) mod transfer_fee;
pub(crate) mod transfer_hook;

/// Mint extension parameters captured by `CreateMintWithExtensions::with_*` methods.
#[derive(Debug, Clone)]
pub(crate) enum MintExtensionInit {
    /// Transfer-fee configuration.
    TransferFee {
        /// Fee basis points.
        fee_basis_points: u16,
        /// Maximum fee.
        maximum_fee: u64,
    },
    /// Transfer hook program id.
    TransferHook { program_id: Pubkey },
    /// Mint close authority.
    MintCloseAuthority { authority: Pubkey },
    /// Metadata pointer address.
    MetadataPointer { metadata_address: Pubkey },
    /// Non-transferable marker.
    NonTransferable,
    /// Permanent delegate.
    PermanentDelegate { delegate: Pubkey },
    /// Interest-bearing configuration.
    InterestBearing {
        /// Rate authority.
        rate_authority: Pubkey,
        /// Current rate.
        rate: i16,
    },
}

/// Account extension parameters captured by `CreateAccountWithExtensions::with_*` methods.
#[derive(Debug, Clone)]
pub(crate) enum AccountExtensionInit {
    /// Pre-init: initialized BEFORE `initialize_account3`. No owner signature needed.
    ImmutableOwner,
    /// Post-init enable: called AFTER `initialize_account3`. Owner must sign.
    MemoTransfer,
    /// Post-init enable: called AFTER `initialize_account3`. Owner must sign.
    CpiGuard,
}
