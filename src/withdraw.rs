use serde::{Deserialize, Serialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserWallet {
    pub balance: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    msg!("Withdraw from PDA wallet");

    let account_info_iter = &mut accounts.iter();

    // The first account should be the user wallet account (PDA)
    let user_wallet_account = next_account_info(account_info_iter)?;

    // Ensure the user wallet account is owned by this program
    if user_wallet_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the user wallet data from the PDA account
    let mut user_wallet_data: UserWallet =
        serde_json::from_slice(&user_wallet_account.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)?;

    // Ensure there are enough funds in the wallet
    if user_wallet_data.balance < amount {
        return Err(ProgramError::Custom(0)); // Use a custom error to indicate insufficient funds
    }

    // Update the balance by subtracting the withdrawal amount
    user_wallet_data.balance -= amount;

    // Serialize the updated user wallet data
    let serialized_data =
        serde_json::to_vec(&user_wallet_data).map_err(|_| ProgramError::InvalidInstructionData)?;

    // Borrow mutable data of the user wallet account and copy the serialized data back into it
    let mut user_wallet_account_data = user_wallet_account.try_borrow_mut_data()?;

    // Ensure the account is large enough to hold the updated serialized data
    if user_wallet_account_data.len() < serialized_data.len() {
        return Err(ProgramError::AccountDataTooSmall);
    }

    // Copy the serialized data into the account's data field
    user_wallet_account_data.copy_from_slice(&serialized_data);

    msg!(
        "Withdrawn {} from PDA wallet. New balance: {}",
        amount,
        user_wallet_data.balance
    );

    Ok(())
}
