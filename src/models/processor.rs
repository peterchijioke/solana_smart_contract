use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};

pub struct Processor;

impl Processor {
    pub fn process_create_asset(accounts: &[AccountInfo], metadata: String) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let asset_account = next_account_info(accounts_iter)?;

        if !asset_account.is_signer {
            msg!("Asset account must be signer");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let asset = Asset {
            owner: *asset_account.key,
            metadata,
        };
        asset.serialize(&mut &mut asset_account.data.borrow_mut()[..])?;
        Ok(())
    }

    pub fn process_mint_asset(accounts: &[AccountInfo], metadata: String) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let asset_account = next_account_info(accounts_iter)?;

        let mut asset = Asset::try_from_slice(&asset_account.data.borrow())?;

        if asset.owner != *asset_account.key {
            msg!("Account does not own this asset");
            return Err(ProgramError::InvalidAccountData);
        }

        asset.metadata = metadata;
        asset.serialize(&mut &mut asset_account.data.borrow_mut()[..])?;
        Ok(())
    }
}
