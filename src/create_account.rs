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

    let payer_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Derive PDA for the user's wallet
    let (pda, bump) = Pubkey::find_program_address(
        &[b"user_wallet", payer_account.key.as_ref()], // Seeds, for unique PDA per user
        program_id,
    );

    // Define space, rent, and lamports for the new PDA account
    let account_space: u64 = 128; // Define the required space (example: 128 bytes)
    let rent_exemption = Rent::default().minimum_balance(account_space as usize);

    // Create PDA account (this will be the "user wallet")
    let create_account_ix = system_instruction::create_account(
        payer_account.key, // Funding account
        &pda,              // The PDA address (user wallet)
        rent_exemption,    // Rent exemption lamports
        account_space,     // Space required for the PDA account
        program_id,        // Owner of the PDA account
    );

    // Add the bump seed for signing the instruction
    let seeds = &[b"user_wallet", payer_account.key.as_ref(), &[bump]];

    // Use `invoke_signed` to sign with the PDA's bump seed
    invoke_signed(
        &create_account_ix,
        &[payer_account.clone(), system_program.clone()],
        &[seeds], // Signing with the PDA
    )?;

    // Initialize the balance of the user wallet to 0
    let user_wallet_data = UserWallet { balance: 0 };

    // Serialize the struct into a Vec<u8>, handling errors explicitly using Borsh
    let user_wallet_data_serialized = user_wallet_data.try_to_vec().map_err(|e| {
        msg!("Serialization error: {}", e);
        ProgramError::InvalidInstructionData // or another appropriate error
    })?;

    // Store the user wallet data in the account (use .borrow_mut_data() to get a mutable reference to account data)
    let user_wallet_account = next_account_info(account_info_iter)?;
    let mut user_wallet_account_data = user_wallet_account.try_borrow_mut_data()?;

    // Ensure that the account is large enough to hold the serialized data
    if user_wallet_account_data.len() < user_wallet_data_serialized.len() {
        return Err(ProgramError::AccountDataTooSmall);
    }

    // Write the serialized data to the PDA account's data field
    user_wallet_account_data.copy_from_slice(&user_wallet_data_serialized);

    msg!(
        "PDA wallet account created successfully with balance: {}",
        user_wallet_data.balance
    );

    Ok(())
}
