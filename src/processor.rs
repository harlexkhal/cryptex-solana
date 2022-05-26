use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    program_error::ProgramError,
};

use crate::{instruction::CryptexInstruction};

pub struct Processor;
impl Processor {
    pub fn process(
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CryptexInstruction::unpack(instruction_data)?;

        match instruction {
            CryptexInstruction::Stake { amount } => {
                msg!("Instruction: InitCryptex");
                Self::process_stake(accounts, amount)
            }
            CryptexInstruction::Mint { amount } => {
                msg!("Instruction: Exchange");
                Self::process_mint(accounts, amount)
            }
        }
    }

    fn process_stake(
        accounts: &[AccountInfo],
        amount: u64
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let token_program_id = next_account_info(account_info_iter)?;
        let destination_pubkey = next_account_info(account_info_iter)?;
        let source_pubkey = next_account_info(account_info_iter)?;
        
        if !source_pubkey.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let transfer_ix = spl_token::instruction::transfer(
            token_program_id.key,
            source_pubkey.key,
            destination_pubkey.key,
            source_pubkey.key,
            &[&source_pubkey.key],
            amount
        )?;

        msg!("Transfering Token...");
        invoke(
            &transfer_ix,
            &[
                token_program_id.clone(),
                source_pubkey.clone(),
                destination_pubkey.clone(),
                source_pubkey.clone()
            ],
        )?;

        Ok(())
    }

    fn process_mint(
        accounts: &[AccountInfo],
        amount: u64
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let token_program_id = next_account_info(account_info_iter)?;
        let destination_account_pubkey = next_account_info(account_info_iter)?;
        let owner_pubkey = next_account_info(account_info_iter)?;
        let mint_token_pubkey = next_account_info(account_info_iter)?;
        let owner_token_pubkey = next_account_info(account_info_iter)?;
        
        if !owner_pubkey.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mint_ix = spl_token::instruction::mint_to(
            token_program_id.key,
            mint_token_pubkey.key,
            destination_account_pubkey.key,
            owner_token_pubkey.key,
            &[&owner_token_pubkey.key],
            amount
        )?;

        msg!("Minting and completing wrapping process...");
        invoke(
            &mint_ix,
            &[
                token_program_id.clone(),
                mint_token_pubkey.clone(),
                destination_account_pubkey.clone(),
                owner_token_pubkey.clone()
            ],
        )?;

        Ok(())
    }
}