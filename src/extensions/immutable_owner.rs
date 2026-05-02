//! `ImmutableOwner` extension support (pre-init).

use litesvm::types::FailedTransactionMetadata;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use spl_token_2022_interface::{extension::ExtensionType, instruction::initialize_immutable_owner};

pub(crate) const EXTENSION_TYPE: ExtensionType = ExtensionType::ImmutableOwner;

pub(crate) fn build_init_instruction(
    account: &Pubkey,
) -> Result<Instruction, FailedTransactionMetadata> {
    Ok(initialize_immutable_owner(
        &spl_token_2022_interface::id(),
        account,
    )?)
}
