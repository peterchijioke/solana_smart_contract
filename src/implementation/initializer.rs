use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

use crate::models::asset::Asset;

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
    pub fn process_sell_asset(
        accounts: &[AccountInfo],
        new_owner: Pubkey,
        sale_amount: u64,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let seller_account = next_account_info(accounts_iter)?;
        let asset_account = next_account_info(accounts_iter)?;
        let buyer_account = next_account_info(accounts_iter)?;
        let system_program_account = next_account_info(accounts_iter)?;

        // Load the asset data from the account
        let mut asset = Asset::try_from_slice(&asset_account.data.borrow())?;

        // Verify that the seller is the owner of the asset
        if asset.owner != *seller_account.key {
            msg!("Seller account does not own this asset");
            return Err(ProgramError::InvalidAccountData);
        }

        // Transfer SOL from buyer to seller as payment for the asset
        let transfer_instruction =
            system_instruction::transfer(&buyer_account.key, &seller_account.key, sale_amount);

        invoke(
            &transfer_instruction,
            &[
                buyer_account.clone(),
                seller_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        // Transfer ownership of the asset to the buyer
        asset.owner = new_owner;
        asset.serialize(&mut &mut asset_account.data.borrow_mut()[..])?;

        Ok(())
    }
    pub fn process_transfer_ownership(
        accounts: &[AccountInfo],
        new_owner: Pubkey,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let asset_account = next_account_info(accounts_iter)?;

        let mut asset = Asset::try_from_slice(&asset_account.data.borrow())?;

        if asset.owner != *asset_account.key {
            msg!("Account does not own this asset");
            return Err(ProgramError::InvalidAccountData);
        }

        asset.owner = new_owner;
        asset.serialize(&mut &mut asset_account.data.borrow_mut()[..])?;
        Ok(())
    }
}
