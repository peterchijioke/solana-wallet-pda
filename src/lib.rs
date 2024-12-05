mod deposit;
mod pool_account;
mod user_account;
mod withdraw;
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (instruction, data) = Instruction::decode(instruction_data)?;
    match instruction {
        Instruction::CreateAccount => user_account::process(program_id, accounts, data),
        Instruction::Deposit(amount) => deposit::process(program_id, accounts, amount),
        Instruction::Withdraw(amount) => withdraw::process(program_id, accounts, amount),
        Instruction::PoolAccount => pool_account::process(program_id, accounts, data),
    }
}

#[derive(Debug)]
pub enum Instruction {
    CreateAccount,
    Deposit(u64),
    Withdraw(u64),
    PoolAccount,
}

impl Instruction {
    pub fn decode(data: &[u8]) -> Result<(Self, &[u8]), ProgramError> {
        if data.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }

        let (instruction_byte, rest) = data.split_first().unwrap();

        let instruction = match instruction_byte {
            0 => Instruction::CreateAccount,
            1 => {
                let amount = u64::from_le_bytes(
                    rest.try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                Instruction::Deposit(amount)
            }
            2 => {
                let amount = u64::from_le_bytes(
                    rest.try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                Instruction::Withdraw(amount)
            }
            3 => Instruction::PoolAccount,

            _ => return Err(ProgramError::InvalidInstructionData),
        };

        Ok((instruction, rest))
    }
}
