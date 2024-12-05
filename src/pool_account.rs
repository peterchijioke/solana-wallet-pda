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
pub struct PoolWallet {
    pub total_balance: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    msg!("Create Pool Wallet PDA instruction called");

    let account_info_iter = &mut accounts.iter();
    let payer_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (pool_wallet_pda, bump) = Pubkey::find_program_address(&[b"pool_wallet"], program_id);

    let seeds: &[&[&[u8]]] = &[&[b"pool_wallet"], &[&bump.to_le_bytes()]];

    let account_space: u64 = 128;
    let rent_exemption = Rent::default().minimum_balance(account_space as usize);

    if **payer_account.lamports.borrow() < rent_exemption {
        msg!("Not enough balance in payer account to cover rent");
        return Err(ProgramError::InsufficientFunds);
    }

    let create_account_ix = system_instruction::create_account(
        payer_account.key,
        &pool_wallet_pda,
        rent_exemption,
        account_space,
        program_id,
    );

    invoke_signed(
        &create_account_ix,
        &[payer_account.clone(), system_program.clone()],
        seeds,
    )?;

    let pool_wallet_data = PoolWallet { total_balance: 0 };
    let pool_wallet_data_serialized = pool_wallet_data.try_to_vec().map_err(|e| {
        msg!("Serialization error: {}", e);
        ProgramError::InvalidInstructionData
    })?;

    let pool_account = next_account_info(account_info_iter)?;
    let mut pool_account_data = pool_account.try_borrow_mut_data()?;

    if pool_account_data.len() < pool_wallet_data_serialized.len() {
        return Err(ProgramError::AccountDataTooSmall);
    }

    pool_account_data.copy_from_slice(&pool_wallet_data_serialized);

    msg!(
        "Pool wallet PDA created successfully with balance: {}",
        pool_wallet_data.total_balance
    );

    Ok(())
}
