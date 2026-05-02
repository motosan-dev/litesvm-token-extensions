//! `TransferHook` extension support.

use litesvm::types::FailedTransactionMetadata;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use spl_token_2022_interface::extension::{transfer_hook, ExtensionType};

pub(crate) const EXTENSION_TYPE: ExtensionType = ExtensionType::TransferHook;

pub(crate) fn build_init_instruction(
    mint: &Pubkey,
    program_id: Pubkey,
) -> Result<Instruction, FailedTransactionMetadata> {
    Ok(transfer_hook::instruction::initialize(
        &spl_token_2022_interface::id(),
        mint,
        None,
        Some(program_id),
    )?)
}
