use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define the UserWallet struct with Borsh serialization
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserWallet {
    pub balance: u64,
}

// Define the PoolWallet struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PoolWallet {
    pub total_balance: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    msg!("Deposit into User Wallet and Pool Wallet");

    let account_info_iter = &mut accounts.iter();

    // The first account should be the user wallet account (PDA)
    let user_wallet_account = next_account_info(account_info_iter)?;

    // The second account should be the pool wallet account
    let pool_wallet_account = next_account_info(account_info_iter)?;

    // Ensure the user wallet account is owned by this program
    if user_wallet_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Ensure the pool wallet account is owned by this program
    if pool_wallet_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the user wallet data from the PDA account
    let mut user_wallet_data: UserWallet =
        UserWallet::try_from_slice(&user_wallet_account.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)?;

    // Deserialize the pool wallet data
    let mut pool_wallet_data: PoolWallet =
        PoolWallet::try_from_slice(&pool_wallet_account.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)?;

    // Check if the pool wallet has enough funds for the deposit
    if pool_wallet_data.total_balance < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    // Update the user's balance
    user_wallet_data.balance += amount;

    // Deduct the funds from the pool wallet
    pool_wallet_data.total_balance -= amount;

    // Serialize the updated user wallet data
    let serialized_user_wallet_data = user_wallet_data
        .try_to_vec()
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Serialize the updated pool wallet data
    let serialized_pool_wallet_data = pool_wallet_data
        .try_to_vec()
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Borrow mutable data of the user wallet account and copy the serialized data back into it
    let mut user_wallet_account_data = user_wallet_account.try_borrow_mut_data()?;

    // Ensure the account is large enough to hold the updated serialized data
    if user_wallet_account_data.len() < serialized_user_wallet_data.len() {
        return Err(ProgramError::AccountDataTooSmall);
    }

    // Copy the serialized user wallet data into the account's data field
    user_wallet_account_data.copy_from_slice(&serialized_user_wallet_data);

    // Borrow mutable data of the pool wallet account and copy the serialized data back into it
    let mut pool_wallet_account_data = pool_wallet_account.try_borrow_mut_data()?;

    // Ensure the account is large enough to hold the updated serialized data
    if pool_wallet_account_data.len() < serialized_pool_wallet_data.len() {
        return Err(ProgramError::AccountDataTooSmall);
    }

    // Copy the serialized pool wallet data into the account's data field
    pool_wallet_account_data.copy_from_slice(&serialized_pool_wallet_data);

    msg!(
        "Deposited {} into user wallet. User's new balance: {}. Pool wallet's new balance: {}",
        amount,
        user_wallet_data.balance,
        pool_wallet_data.total_balance
    );

    Ok(())
}
