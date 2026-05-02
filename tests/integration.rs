//! Integration tests for litesvm-token-extensions using LiteSVM.

use litesvm::LiteSVM;
use litesvm_token_extensions::{CreateAccountWithExtensions, CreateMintWithExtensions};
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use spl_token_2022_interface::{
    extension::{
        cpi_guard::CpiGuard, immutable_owner::ImmutableOwner,
        interest_bearing_mint::InterestBearingConfig, memo_transfer::MemoTransfer,
        metadata_pointer::MetadataPointer, mint_close_authority::MintCloseAuthority,
        non_transferable::NonTransferable, permanent_delegate::PermanentDelegate,
        transfer_fee::TransferFeeConfig, transfer_hook::TransferHook, BaseStateWithExtensions,
        StateWithExtensions,
    },
    state::{Account as TokenAccount, Mint},
};

fn setup() -> (LiteSVM, Keypair) {
    let mut svm = LiteSVM::new();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 100_000_000_000).unwrap();
    (svm, payer)
}

fn unpack_mint(svm: &LiteSVM, mint: Pubkey) -> Mint {
    let account = svm.get_account(&mint).expect("mint missing");
    StateWithExtensions::<Mint>::unpack(&account.data)
        .expect("unpack failed")
        .base
}

fn unpack_mint_with_extensions(svm: &LiteSVM, mint: Pubkey) -> (Mint, Vec<u8>) {
    let account = svm.get_account(&mint).expect("mint missing");
    let unpacked = StateWithExtensions::<Mint>::unpack(&account.data).expect("unpack failed");
    (unpacked.base, account.data)
}

fn create_vanilla_mint(svm: &mut LiteSVM, payer: &Keypair, decimals: u8) -> Pubkey {
    CreateMintWithExtensions::new(svm, payer)
        .decimals(decimals)
        .send()
        .unwrap()
}

fn unpack_account_with_extensions(svm: &LiteSVM, account: Pubkey) -> Vec<u8> {
    svm.get_account(&account).expect("account missing").data
}

#[test]
fn mint_with_transfer_fee_writes_extension_data() {
    let (mut svm, payer) = setup();
    let mint = CreateMintWithExtensions::new(&mut svm, &payer)
        .decimals(6)
        .with_transfer_fee(250, 5_000_000)
        .send()
        .unwrap();

    let (base, data) = unpack_mint_with_extensions(&svm, mint);
    assert_eq!(base.decimals, 6);
    let unpacked = StateWithExtensions::<Mint>::unpack(&data).unwrap();
    let cfg = unpacked.get_extension::<TransferFeeConfig>().unwrap();
    let bps: u16 = cfg.newer_transfer_fee.transfer_fee_basis_points.into();
    let max: u64 = cfg.newer_transfer_fee.maximum_fee.into();
    assert_eq!(bps, 250);
    assert_eq!(max, 5_000_000);
}

#[test]
fn mint_with_transfer_hook_writes_program_id() {
    let (mut svm, payer) = setup();
    let hook_program = Pubkey::new_unique();
    let mint = CreateMintWithExtensions::new(&mut svm, &payer)
        .decimals(9)
        .with_transfer_hook(hook_program)
        .send()
        .unwrap();

    let (_, data) = unpack_mint_with_extensions(&svm, mint);
    let unpacked = StateWithExtensions::<Mint>::unpack(&data).unwrap();
    let hook = unpacked.get_extension::<TransferHook>().unwrap();
    let program_id = Option::<Pubkey>::from(hook.program_id).unwrap();
    assert_eq!(program_id, hook_program);
}

#[test]
fn mint_with_close_authority_writes_authority() {
    let (mut svm, payer) = setup();
    let close_auth = Pubkey::new_unique();
    let mint = CreateMintWithExtensions::new(&mut svm, &payer)
        .with_mint_close_authority(close_auth)
        .send()
        .unwrap();

    let (_, data) = unpack_mint_with_extensions(&svm, mint);
    let unpacked = StateWithExtensions::<Mint>::unpack(&data).unwrap();
    let ext = unpacked.get_extension::<MintCloseAuthority>().unwrap();
    let actual = Option::<Pubkey>::from(ext.close_authority).unwrap();
    assert_eq!(actual, close_auth);
}

#[test]
fn mint_with_metadata_pointer_writes_address() {
    let (mut svm, payer) = setup();
    let metadata_addr = Pubkey::new_unique();
    let mint = CreateMintWithExtensions::new(&mut svm, &payer)
        .with_metadata_pointer(metadata_addr)
        .send()
        .unwrap();

    let (_, data) = unpack_mint_with_extensions(&svm, mint);
    let unpacked = StateWithExtensions::<Mint>::unpack(&data).unwrap();
    let ptr = unpacked.get_extension::<MetadataPointer>().unwrap();
    let actual = Option::<Pubkey>::from(ptr.metadata_address).unwrap();
    assert_eq!(actual, metadata_addr);
}

#[test]
fn mint_with_non_transferable_marks_extension() {
    let (mut svm, payer) = setup();
    let mint = CreateMintWithExtensions::new(&mut svm, &payer)
        .with_non_transferable()
        .send()
        .unwrap();

    let (_, data) = unpack_mint_with_extensions(&svm, mint);
    let unpacked = StateWithExtensions::<Mint>::unpack(&data).unwrap();
    let _: &NonTransferable = unpacked.get_extension::<NonTransferable>().unwrap();
}

#[test]
fn mint_with_permanent_delegate_writes_delegate() {
    let (mut svm, payer) = setup();
    let delegate = Pubkey::new_unique();
    let mint = CreateMintWithExtensions::new(&mut svm, &payer)
        .with_permanent_delegate(delegate)
        .send()
        .unwrap();

    let (_, data) = unpack_mint_with_extensions(&svm, mint);
    let unpacked = StateWithExtensions::<Mint>::unpack(&data).unwrap();
    let ext = unpacked.get_extension::<PermanentDelegate>().unwrap();
    let actual = Option::<Pubkey>::from(ext.delegate).unwrap();
    assert_eq!(actual, delegate);
}

#[test]
fn mint_with_interest_bearing_writes_rate() {
    let (mut svm, payer) = setup();
    let rate_authority = Pubkey::new_unique();
    let mint = CreateMintWithExtensions::new(&mut svm, &payer)
        .with_interest_bearing(rate_authority, 100)
        .send()
        .unwrap();

    let (_, data) = unpack_mint_with_extensions(&svm, mint);
    let unpacked = StateWithExtensions::<Mint>::unpack(&data).unwrap();
    let ext = unpacked.get_extension::<InterestBearingConfig>().unwrap();
    let rate: i16 = ext.current_rate.into();
    assert_eq!(rate, 100);
    let actual_authority = Option::<Pubkey>::from(ext.rate_authority).unwrap();
    assert_eq!(actual_authority, rate_authority);
}

#[test]
fn account_with_immutable_owner_marks_extension() {
    let (mut svm, payer) = setup();
    let mint = create_vanilla_mint(&mut svm, &payer, 9);

    let account = CreateAccountWithExtensions::new(&mut svm, &payer, &mint)
        .with_immutable_owner()
        .send()
        .unwrap();

    let data = unpack_account_with_extensions(&svm, account);
    let unpacked = StateWithExtensions::<TokenAccount>::unpack(&data).unwrap();
    let _: &ImmutableOwner = unpacked.get_extension::<ImmutableOwner>().unwrap();
}

#[test]
fn account_with_memo_transfer_marks_required_memos_enabled() {
    let (mut svm, payer) = setup();
    let mint = create_vanilla_mint(&mut svm, &payer, 9);

    let account = CreateAccountWithExtensions::new(&mut svm, &payer, &mint)
        .with_memo_transfer()
        .send()
        .unwrap();

    let data = unpack_account_with_extensions(&svm, account);
    let unpacked = StateWithExtensions::<TokenAccount>::unpack(&data).unwrap();
    let ext = unpacked.get_extension::<MemoTransfer>().unwrap();
    assert!(bool::from(ext.require_incoming_transfer_memos));
}

#[test]
fn account_with_cpi_guard_marks_lock_privilege_enabled() {
    let (mut svm, payer) = setup();
    let mint = create_vanilla_mint(&mut svm, &payer, 9);

    let account = CreateAccountWithExtensions::new(&mut svm, &payer, &mint)
        .with_cpi_guard()
        .send()
        .unwrap();

    let data = unpack_account_with_extensions(&svm, account);
    let unpacked = StateWithExtensions::<TokenAccount>::unpack(&data).unwrap();
    let ext = unpacked.get_extension::<CpiGuard>().unwrap();
    assert!(bool::from(ext.lock_cpi));
}

#[test]
fn mint_with_three_extensions_coexist() {
    let (mut svm, payer) = setup();
    let close_auth = Pubkey::new_unique();
    let metadata_addr = Pubkey::new_unique();
    let mint = CreateMintWithExtensions::new(&mut svm, &payer)
        .decimals(9)
        .with_transfer_fee(100, 1_000_000)
        .with_metadata_pointer(metadata_addr)
        .with_mint_close_authority(close_auth)
        .send()
        .unwrap();

    let (_, data) = unpack_mint_with_extensions(&svm, mint);
    let unpacked = StateWithExtensions::<Mint>::unpack(&data).unwrap();
    let _ = unpacked.get_extension::<TransferFeeConfig>().unwrap();
    let _ = unpacked.get_extension::<MetadataPointer>().unwrap();
    let _ = unpacked.get_extension::<MintCloseAuthority>().unwrap();
}

#[test]
fn account_with_pre_and_post_init_extensions_coexist() {
    let (mut svm, payer) = setup();
    let mint = create_vanilla_mint(&mut svm, &payer, 6);

    let account = CreateAccountWithExtensions::new(&mut svm, &payer, &mint)
        .with_immutable_owner()
        .with_memo_transfer()
        .send()
        .unwrap();

    let data = unpack_account_with_extensions(&svm, account);
    let unpacked = StateWithExtensions::<TokenAccount>::unpack(&data).unwrap();
    let _ = unpacked.get_extension::<ImmutableOwner>().unwrap();
    let memo_ext = unpacked.get_extension::<MemoTransfer>().unwrap();
    assert!(bool::from(memo_ext.require_incoming_transfer_memos));
}

#[test]
fn mint_default_decimals_and_authority_match_payer() {
    let (mut svm, payer) = setup();
    let mint = CreateMintWithExtensions::new(&mut svm, &payer)
        .send()
        .unwrap();

    let base = unpack_mint(&svm, mint);
    assert_eq!(base.decimals, 9);
    let mint_authority = Option::<Pubkey>::from(base.mint_authority).unwrap();
    assert_eq!(mint_authority, payer.pubkey());
}

#[test]
fn account_default_owner_matches_payer() {
    let (mut svm, payer) = setup();
    let mint = create_vanilla_mint(&mut svm, &payer, 9);

    let account_pk = CreateAccountWithExtensions::new(&mut svm, &payer, &mint)
        .send()
        .unwrap();

    let data = unpack_account_with_extensions(&svm, account_pk);
    let unpacked = StateWithExtensions::<TokenAccount>::unpack(&data).unwrap();
    assert_eq!(unpacked.base.owner, payer.pubkey());
    assert_eq!(unpacked.base.mint, mint);
}

#[test]
fn mint_with_zero_extensions_succeeds_as_vanilla() {
    let (mut svm, payer) = setup();
    let mint = CreateMintWithExtensions::new(&mut svm, &payer)
        .decimals(2)
        .send()
        .unwrap();

    let base = unpack_mint(&svm, mint);
    assert_eq!(base.decimals, 2);
    assert!(base.is_initialized);
}

#[test]
fn account_with_zero_extensions_succeeds_as_vanilla() {
    let (mut svm, payer) = setup();
    let mint = create_vanilla_mint(&mut svm, &payer, 9);

    let account_pk = CreateAccountWithExtensions::new(&mut svm, &payer, &mint)
        .send()
        .unwrap();

    let data = unpack_account_with_extensions(&svm, account_pk);
    let unpacked = StateWithExtensions::<TokenAccount>::unpack(&data).unwrap();
    assert_eq!(unpacked.base.owner, payer.pubkey());
}
