//! `CreateMintWithExtensions` builder.

use litesvm::{types::FailedTransactionMetadata, LiteSVM};
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_system_interface::instruction as system_instruction;
use solana_transaction::Transaction;
use spl_token_2022_interface::{
    extension::ExtensionType, instruction::initialize_mint2, state::Mint,
};

use crate::extensions::{
    interest_bearing, metadata_pointer, mint_close_authority, non_transferable, permanent_delegate,
    transfer_fee, transfer_hook, MintExtensionInit,
};

/// Builder for creating a Token-2022 mint with one or more extensions.
///
/// Mirrors `litesvm_token::CreateMint`'s API style. Extensions are added via
/// chainable `.with_*()` methods and initialized in the order they were added,
/// followed by the base `initialize_mint2`.
///
/// # Example
///
/// ```no_run
/// # use litesvm::LiteSVM;
/// # use litesvm_token_extensions::CreateMintWithExtensions;
/// # use solana_keypair::Keypair;
/// # use solana_signer::Signer;
/// # let mut svm = LiteSVM::new();
/// # let payer = Keypair::new();
/// let mint = CreateMintWithExtensions::new(&mut svm, &payer)
///     .decimals(6)
///     .with_transfer_fee(250, 5_000_000)
///     .with_metadata_pointer(payer.pubkey())
///     .send()
///     .unwrap();
/// ```
pub struct CreateMintWithExtensions<'a> {
    svm: &'a mut LiteSVM,
    payer: &'a Keypair,
    authority: Option<Pubkey>,
    freeze_authority: Option<Pubkey>,
    decimals: u8,
    extensions: Vec<MintExtensionInit>,
}

impl<'a> CreateMintWithExtensions<'a> {
    /// Create a new builder.
    pub fn new(svm: &'a mut LiteSVM, payer: &'a Keypair) -> Self {
        Self {
            svm,
            payer,
            authority: None,
            freeze_authority: None,
            decimals: 9,
            extensions: Vec::new(),
        }
    }

    /// Override the mint authority (defaults to payer).
    pub fn authority(mut self, authority: Pubkey) -> Self {
        self.authority = Some(authority);
        self
    }

    /// Override the freeze authority (defaults to None).
    pub fn freeze_authority(mut self, freeze_authority: Pubkey) -> Self {
        self.freeze_authority = Some(freeze_authority);
        self
    }

    /// Set the mint decimals (defaults to 9).
    pub fn decimals(mut self, decimals: u8) -> Self {
        self.decimals = decimals;
        self
    }

    /// Add the `TransferFeeConfig` extension.
    pub fn with_transfer_fee(mut self, fee_basis_points: u16, maximum_fee: u64) -> Self {
        self.extensions.push(MintExtensionInit::TransferFee {
            fee_basis_points,
            maximum_fee,
        });
        self
    }

    /// Add the `TransferHook` extension.
    pub fn with_transfer_hook(mut self, program_id: Pubkey) -> Self {
        self.extensions
            .push(MintExtensionInit::TransferHook { program_id });
        self
    }

    /// Add the `MintCloseAuthority` extension.
    pub fn with_mint_close_authority(mut self, authority: Pubkey) -> Self {
        self.extensions
            .push(MintExtensionInit::MintCloseAuthority { authority });
        self
    }

    /// Add the `MetadataPointer` extension.
    pub fn with_metadata_pointer(mut self, metadata_address: Pubkey) -> Self {
        self.extensions
            .push(MintExtensionInit::MetadataPointer { metadata_address });
        self
    }

    /// Add the `NonTransferable` extension.
    pub fn with_non_transferable(mut self) -> Self {
        self.extensions.push(MintExtensionInit::NonTransferable);
        self
    }

    /// Add the `PermanentDelegate` extension.
    pub fn with_permanent_delegate(mut self, delegate: Pubkey) -> Self {
        self.extensions
            .push(MintExtensionInit::PermanentDelegate { delegate });
        self
    }

    /// Add the `InterestBearingConfig` extension.
    pub fn with_interest_bearing(mut self, rate_authority: Pubkey, rate: i16) -> Self {
        self.extensions.push(MintExtensionInit::InterestBearing {
            rate_authority,
            rate,
        });
        self
    }

    /// Build and submit the create_account + extension-init + initialize_mint2 sequence.
    pub fn send(self) -> Result<Pubkey, FailedTransactionMetadata> {
        let mint_kp = Keypair::new();
        let mint_pk = mint_kp.pubkey();

        let extension_types: Vec<ExtensionType> = self
            .extensions
            .iter()
            .map(|ext| match ext {
                MintExtensionInit::TransferFee { .. } => transfer_fee::EXTENSION_TYPE,
                MintExtensionInit::TransferHook { .. } => transfer_hook::EXTENSION_TYPE,
                MintExtensionInit::MintCloseAuthority { .. } => {
                    mint_close_authority::EXTENSION_TYPE
                }
                MintExtensionInit::MetadataPointer { .. } => metadata_pointer::EXTENSION_TYPE,
                MintExtensionInit::NonTransferable => non_transferable::EXTENSION_TYPE,
                MintExtensionInit::PermanentDelegate { .. } => permanent_delegate::EXTENSION_TYPE,
                MintExtensionInit::InterestBearing { .. } => interest_bearing::EXTENSION_TYPE,
            })
            .collect();

        let mint_size = ExtensionType::try_calculate_account_len::<Mint>(&extension_types)?;
        let rent = self.svm.minimum_balance_for_rent_exemption(mint_size);
        let token_program = spl_token_2022_interface::id();

        let mut instructions = Vec::with_capacity(self.extensions.len() + 2);
        instructions.push(system_instruction::create_account(
            &self.payer.pubkey(),
            &mint_pk,
            rent,
            mint_size as u64,
            &token_program,
        ));

        for ext in &self.extensions {
            let ix = match ext {
                MintExtensionInit::TransferFee {
                    fee_basis_points,
                    maximum_fee,
                } => {
                    transfer_fee::build_init_instruction(&mint_pk, *fee_basis_points, *maximum_fee)?
                }
                MintExtensionInit::TransferHook { program_id } => {
                    transfer_hook::build_init_instruction(&mint_pk, *program_id)?
                }
                MintExtensionInit::MintCloseAuthority { authority } => {
                    mint_close_authority::build_init_instruction(&mint_pk, *authority)?
                }
                MintExtensionInit::MetadataPointer { metadata_address } => {
                    metadata_pointer::build_init_instruction(&mint_pk, *metadata_address)?
                }
                MintExtensionInit::NonTransferable => {
                    non_transferable::build_init_instruction(&mint_pk)?
                }
                MintExtensionInit::PermanentDelegate { delegate } => {
                    permanent_delegate::build_init_instruction(&mint_pk, *delegate)?
                }
                MintExtensionInit::InterestBearing {
                    rate_authority,
                    rate,
                } => interest_bearing::build_init_instruction(&mint_pk, *rate_authority, *rate)?,
            };
            instructions.push(ix);
        }

        let authority_pk = self.authority.unwrap_or_else(|| self.payer.pubkey());
        instructions.push(initialize_mint2(
            &token_program,
            &mint_pk,
            &authority_pk,
            self.freeze_authority.as_ref(),
            self.decimals,
        )?);

        let tx = Transaction::new_signed_with_payer(
            &instructions,
            Some(&self.payer.pubkey()),
            &[self.payer, &mint_kp],
            self.svm.latest_blockhash(),
        );
        self.svm.send_transaction(tx)?;
        Ok(mint_pk)
    }
}
