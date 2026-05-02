//! `TransferFeeConfig` extension support.

use litesvm::types::FailedTransactionMetadata;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use spl_token_2022_interface::extension::{transfer_fee, ExtensionType};

pub(crate) const EXTENSION_TYPE: ExtensionType = ExtensionType::TransferFeeConfig;

pub(crate) fn build_init_instruction(
    mint: &Pubkey,
    fee_basis_points: u16,
    maximum_fee: u64,
) -> Result<Instruction, FailedTransactionMetadata> {
    Ok(transfer_fee::instruction::initialize_transfer_fee_config(
        &spl_token_2022_interface::id(),
        mint,
        None,
        None,
        fee_basis_points,
        maximum_fee,
    )?)
}
