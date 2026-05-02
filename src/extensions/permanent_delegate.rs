//! `PermanentDelegate` extension support.

use litesvm::types::FailedTransactionMetadata;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use spl_token_2022_interface::{
    extension::ExtensionType, instruction::initialize_permanent_delegate,
};

pub(crate) const EXTENSION_TYPE: ExtensionType = ExtensionType::PermanentDelegate;

pub(crate) fn build_init_instruction(
    mint: &Pubkey,
    delegate: Pubkey,
) -> Result<Instruction, FailedTransactionMetadata> {
    Ok(initialize_permanent_delegate(
        &spl_token_2022_interface::id(),
        mint,
        &delegate,
    )?)
}
