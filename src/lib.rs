use implementation::initializer::Processor;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};
use std::convert::TryInto;

mod implementation;
mod models;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (instruction, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match instruction {
        0 => {
            let asset_data = String::from_utf8(data.to_vec())
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            Processor::process_create_asset(accounts, asset_data)
        }
        1 => {
            let asset_data = String::from_utf8(data.to_vec())
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            Processor::process_mint_asset(accounts, asset_data)
        }
        2 => {
            if data.len() < 32 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let new_owner = Pubkey::new_from_array(
                data[0..32]
                    .try_into()
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
            );
            Processor::process_transfer_ownership(accounts, new_owner)
        }
        3 => {
            if data.len() < 41 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let new_owner = Pubkey::new_from_array(
                data[0..32]
                    .try_into()
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
            );
            let sale_amount = u64::from_le_bytes(
                data[32..40]
                    .try_into()
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
            );
            Processor::process_sell_asset(accounts, new_owner, sale_amount)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
