use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserWallet {
    pub balance: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PoolWallet {
    pub total_balance: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    msg!("Deposit into User Wallet and Pool Wallet");

    let account_info_iter = &mut accounts.iter();
    let user_wallet_account = next_account_info(account_info_iter)?;
    let pool_wallet_account = next_account_info(account_info_iter)?;

    if user_wallet_account.owner != program_id || pool_wallet_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut user_wallet_data: UserWallet =
        UserWallet::try_from_slice(&user_wallet_account.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)?;

    let mut pool_wallet_data: PoolWallet =
        PoolWallet::try_from_slice(&pool_wallet_account.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)?;

    if pool_wallet_data.total_balance < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    user_wallet_data.balance += amount;
    pool_wallet_data.total_balance -= amount;

    let serialized_user_wallet_data = user_wallet_data
        .try_to_vec()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let serialized_pool_wallet_data = pool_wallet_data
        .try_to_vec()
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let mut user_wallet_account_data = user_wallet_account.try_borrow_mut_data()?;
    if user_wallet_account_data.len() < serialized_user_wallet_data.len() {
        return Err(ProgramError::AccountDataTooSmall);
    }
    user_wallet_account_data.copy_from_slice(&serialized_user_wallet_data);

    let mut pool_wallet_account_data = pool_wallet_account.try_borrow_mut_data()?;
    if pool_wallet_account_data.len() < serialized_pool_wallet_data.len() {
        return Err(ProgramError::AccountDataTooSmall);
    }
    pool_wallet_account_data.copy_from_slice(&serialized_pool_wallet_data);

    msg!(
        "Deposited {} into user wallet. User's new balance: {}. Pool wallet's new balance: {}",
        amount,
        user_wallet_data.balance,
        pool_wallet_data.total_balance
    );

    Ok(())
}
