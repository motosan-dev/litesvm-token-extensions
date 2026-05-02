//! `NonTransferable` extension support.

use litesvm::types::FailedTransactionMetadata;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use spl_token_2022_interface::{
    extension::ExtensionType, instruction::initialize_non_transferable_mint,
};

pub(crate) const EXTENSION_TYPE: ExtensionType = ExtensionType::NonTransferable;

pub(crate) fn build_init_instruction(
    mint: &Pubkey,
) -> Result<Instruction, FailedTransactionMetadata> {
    Ok(initialize_non_transferable_mint(
        &spl_token_2022_interface::id(),
        mint,
    )?)
}
