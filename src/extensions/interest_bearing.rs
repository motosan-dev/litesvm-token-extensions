//! `InterestBearingConfig` extension support.

use litesvm::types::FailedTransactionMetadata;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use spl_token_2022_interface::extension::{interest_bearing_mint, ExtensionType};

pub(crate) const EXTENSION_TYPE: ExtensionType = ExtensionType::InterestBearingConfig;

pub(crate) fn build_init_instruction(
    mint: &Pubkey,
    rate_authority: Pubkey,
    rate: i16,
) -> Result<Instruction, FailedTransactionMetadata> {
    Ok(interest_bearing_mint::instruction::initialize(
        &spl_token_2022_interface::id(),
        mint,
        Some(rate_authority),
        rate,
    )?)
}
