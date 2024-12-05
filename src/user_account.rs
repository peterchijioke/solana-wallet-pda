use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserWallet {
    pub balance: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    msg!("CreateAccount with PDA instruction called");

    let account_info_iter = &mut accounts.iter();

    let pool_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (pda, bump) =
        Pubkey::find_program_address(&[b"user_wallet", pool_account.key.as_ref()], program_id);

    let account_space: u64 = 128;
    let rent_exemption = Rent::default().minimum_balance(account_space as usize);

    if **pool_account.lamports.borrow() < rent_exemption {
        msg!("Not enough balance in pool account to cover rent");
        return Err(ProgramError::InsufficientFunds);
    }

    let create_account_ix = system_instruction::create_account(
        pool_account.key,
        &pda,
        rent_exemption,
        account_space,
        program_id,
    );

    let seeds = &[b"user_wallet", pool_account.key.as_ref(), &[bump]];

    invoke_signed(
        &create_account_ix,
        &[pool_account.clone(), system_program.clone()],
        &[seeds],
    )?;

    let user_wallet_data = UserWallet { balance: 0 };

    let user_wallet_data_serialized = user_wallet_data.try_to_vec().map_err(|e| {
        msg!("Serialization error: {}", e);
        ProgramError::InvalidInstructionData
    })?;

    let user_wallet_account = next_account_info(account_info_iter)?;
    let mut user_wallet_account_data = user_wallet_account.try_borrow_mut_data()?;

    if user_wallet_account_data.len() < user_wallet_data_serialized.len() {
        return Err(ProgramError::AccountDataTooSmall);
    }

    user_wallet_account_data.copy_from_slice(&user_wallet_data_serialized);

    msg!(
        "User wallet account created successfully with balance: {}",
        user_wallet_data.balance
    );

    Ok(())
}
