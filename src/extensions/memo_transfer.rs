//! `MemoTransfer` extension support (post-init enable).

use litesvm::types::FailedTransactionMetadata;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use spl_token_2022_interface::extension::{memo_transfer, ExtensionType};

pub(crate) const EXTENSION_TYPE: ExtensionType = ExtensionType::MemoTransfer;

pub(crate) fn build_enable_instruction(
    account: &Pubkey,
    owner: &Pubkey,
) -> Result<Instruction, FailedTransactionMetadata> {
    Ok(memo_transfer::instruction::enable_required_transfer_memos(
        &spl_token_2022_interface::id(),
        account,
        owner,
        &[],
    )?)
}
