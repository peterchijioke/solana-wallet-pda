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
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?; // User's wallet
    let pool_wallet_account = next_account_info(accounts_iter)?; // Pool wallet account

    // Check if accounts are owned by the correct program
    if *user_account.owner != *program_id || *pool_wallet_account.owner != *program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the pool wallet to check the current balance
    let mut pool_wallet_data = PoolWallet::try_from_slice(&pool_wallet_account.data.borrow())?;

    // Ensure there's enough funds in the pool
    if pool_wallet_data.total_balance < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    // Deduct the funds from the pool wallet
    pool_wallet_data.total_balance -= amount;

    // Serialize updated pool balance back to account
    pool_wallet_data.serialize(&mut *pool_wallet_account.data.borrow_mut())?;

    // Send funds to user
    **user_account.try_borrow_mut_lamports()? += amount;

    msg!("Withdrew {} from the pool wallet", amount);
    Ok(())
}
