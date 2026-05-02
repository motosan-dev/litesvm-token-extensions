//! `MetadataPointer` extension support.

use litesvm::types::FailedTransactionMetadata;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use spl_token_2022_interface::extension::{metadata_pointer, ExtensionType};

pub(crate) const EXTENSION_TYPE: ExtensionType = ExtensionType::MetadataPointer;

pub(crate) fn build_init_instruction(
    mint: &Pubkey,
    metadata_address: Pubkey,
) -> Result<Instruction, FailedTransactionMetadata> {
    Ok(metadata_pointer::instruction::initialize(
        &spl_token_2022_interface::id(),
        mint,
        None,
        Some(metadata_address),
    )?)
}
