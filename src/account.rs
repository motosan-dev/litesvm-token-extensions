//! `CreateAccountWithExtensions` builder.

use litesvm::{types::FailedTransactionMetadata, LiteSVM};
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_system_interface::instruction as system_instruction;
use solana_transaction::Transaction;
use spl_token_2022_interface::{
    extension::ExtensionType, instruction::initialize_account3, state::Account as TokenAccount,
};

use crate::extensions::{cpi_guard, immutable_owner, memo_transfer, AccountExtensionInit};

/// Builder for creating a Token-2022 token account with one or more extensions.
///
/// This creates a non-ATA account using an ephemeral keypair.
///
/// # Example
///
/// ```no_run
/// # use litesvm::LiteSVM;
/// # use litesvm_token_extensions::CreateAccountWithExtensions;
/// # use solana_keypair::Keypair;
/// # let mut svm = LiteSVM::new();
/// # let payer = Keypair::new();
/// # let mint = solana_pubkey::Pubkey::new_unique();
/// let account = CreateAccountWithExtensions::new(&mut svm, &payer, &mint)
///     .with_immutable_owner()
///     .with_memo_transfer()
///     .send()
///     .unwrap();
/// ```
pub struct CreateAccountWithExtensions<'a> {
    svm: &'a mut LiteSVM,
    payer: &'a Keypair,
    mint: &'a Pubkey,
    owner: Option<&'a Keypair>,
    extensions: Vec<AccountExtensionInit>,
}

impl<'a> CreateAccountWithExtensions<'a> {
    /// Create a new builder. Defaults: owner = payer, no extensions.
    pub fn new(svm: &'a mut LiteSVM, payer: &'a Keypair, mint: &'a Pubkey) -> Self {
        Self {
            svm,
            payer,
            mint,
            owner: None,
            extensions: Vec::new(),
        }
    }

    /// Override the account owner.
    pub fn owner(mut self, owner: &'a Keypair) -> Self {
        self.owner = Some(owner);
        self
    }

    /// Add the `ImmutableOwner` extension (pre-init).
    pub fn with_immutable_owner(mut self) -> Self {
        self.extensions.push(AccountExtensionInit::ImmutableOwner);
        self
    }

    /// Add the `MemoTransfer` extension (post-init enable; owner-signed).
    pub fn with_memo_transfer(mut self) -> Self {
        self.extensions.push(AccountExtensionInit::MemoTransfer);
        self
    }

    /// Add the `CpiGuard` extension (post-init enable; owner-signed).
    pub fn with_cpi_guard(mut self) -> Self {
        self.extensions.push(AccountExtensionInit::CpiGuard);
        self
    }

    /// Build and submit the create_account + pre-init + initialize_account3 + post-init sequence.
    pub fn send(self) -> Result<Pubkey, FailedTransactionMetadata> {
        let account_kp = Keypair::new();
        let account_pk = account_kp.pubkey();

        let owner_pk = self
            .owner
            .map(Signer::pubkey)
            .unwrap_or_else(|| self.payer.pubkey());

        let extension_types: Vec<ExtensionType> = self
            .extensions
            .iter()
            .map(|ext| match ext {
                AccountExtensionInit::ImmutableOwner => immutable_owner::EXTENSION_TYPE,
                AccountExtensionInit::MemoTransfer => memo_transfer::EXTENSION_TYPE,
                AccountExtensionInit::CpiGuard => cpi_guard::EXTENSION_TYPE,
            })
            .collect();

        let account_size =
            ExtensionType::try_calculate_account_len::<TokenAccount>(&extension_types)?;
        let rent = self.svm.minimum_balance_for_rent_exemption(account_size);
        let token_program = spl_token_2022_interface::id();

        let mut instructions = Vec::with_capacity(self.extensions.len() + 2);
        instructions.push(system_instruction::create_account(
            &self.payer.pubkey(),
            &account_pk,
            rent,
            account_size as u64,
            &token_program,
        ));

        for ext in &self.extensions {
            if matches!(ext, AccountExtensionInit::ImmutableOwner) {
                instructions.push(immutable_owner::build_init_instruction(&account_pk)?);
            }
        }

        instructions.push(initialize_account3(
            &token_program,
            &account_pk,
            self.mint,
            &owner_pk,
        )?);

        for ext in &self.extensions {
            match ext {
                AccountExtensionInit::ImmutableOwner => {}
                AccountExtensionInit::MemoTransfer => instructions.push(
                    memo_transfer::build_enable_instruction(&account_pk, &owner_pk)?,
                ),
                AccountExtensionInit::CpiGuard => {
                    instructions.push(cpi_guard::build_enable_instruction(&account_pk, &owner_pk)?);
                }
            }
        }

        let needs_owner_signer = self.extensions.iter().any(|ext| {
            matches!(
                ext,
                AccountExtensionInit::MemoTransfer | AccountExtensionInit::CpiGuard
            )
        }) && self
            .owner
            .is_some_and(|owner| owner.pubkey() != self.payer.pubkey());

        let blockhash = self.svm.latest_blockhash();
        let tx = if needs_owner_signer {
            let owner = self.owner.expect("owner checked above");
            Transaction::new_signed_with_payer(
                &instructions,
                Some(&self.payer.pubkey()),
                &[self.payer, &account_kp, owner],
                blockhash,
            )
        } else {
            Transaction::new_signed_with_payer(
                &instructions,
                Some(&self.payer.pubkey()),
                &[self.payer, &account_kp],
                blockhash,
            )
        };

        self.svm.send_transaction(tx)?;
        Ok(account_pk)
    }
}
