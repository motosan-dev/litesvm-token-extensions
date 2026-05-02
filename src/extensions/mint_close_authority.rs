//! `MintCloseAuthority` extension support.

use litesvm::types::FailedTransactionMetadata;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use spl_token_2022_interface::{
    extension::ExtensionType, instruction::initialize_mint_close_authority,
};

pub(crate) const EXTENSION_TYPE: ExtensionType = ExtensionType::MintCloseAuthority;

pub(crate) fn build_init_instruction(
    mint: &Pubkey,
    authority: Pubkey,
) -> Result<Instruction, FailedTransactionMetadata> {
    Ok(initialize_mint_close_authority(
        &spl_token_2022_interface::id(),
        mint,
        Some(&authority),
    )?)
}
