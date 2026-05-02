//! `CpiGuard` extension support (post-init enable).

use litesvm::types::FailedTransactionMetadata;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use spl_token_2022_interface::extension::{cpi_guard, ExtensionType};

pub(crate) const EXTENSION_TYPE: ExtensionType = ExtensionType::CpiGuard;

pub(crate) fn build_enable_instruction(
    account: &Pubkey,
    owner: &Pubkey,
) -> Result<Instruction, FailedTransactionMetadata> {
    Ok(cpi_guard::instruction::enable_cpi_guard(
        &spl_token_2022_interface::id(),
        account,
        owner,
        &[],
    )?)
}
