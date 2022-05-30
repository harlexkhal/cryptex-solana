use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    pubkey::Pubkey,
    program_error::ProgramError,
};

use crate::{instruction::CryptexInstruction};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CryptexInstruction::unpack(instruction_data)?;

        match instruction {
            CryptexInstruction::Stake { amount } => {
                msg!("Instruction: InitCryptex");
                Self::process_stake(program_id, accounts, amount)
            }
            CryptexInstruction::Mint { amount } => {
                msg!("Instruction: Exchange");
                Self::process_mint(program_id, accounts, amount)
            }
        }
    }

    fn process_stake(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64
    ) -> ProgramResult {
        msg!("got into stake");
        let account_info_iter = &mut accounts.iter();
        let token_program_id = next_account_info(account_info_iter)?;
        let destination_pubkey = next_account_info(account_info_iter)?;
        let source_pubkey = next_account_info(account_info_iter)?;
        let signer_pubkey = next_account_info(account_info_iter)?;

 

        msg!("got here");
        
        if !signer_pubkey.is_signer {
            msg!("Failed");
            return Err(ProgramError::MissingRequiredSignature);
        }
        msg!("After error check");
        let transfer_ix = spl_token::instruction::transfer(
            token_program_id.key,
            source_pubkey.key,
            destination_pubkey.key,
            signer_pubkey.key,
            &[&signer_pubkey.key],
            amount
        )?;

        msg!("Transfering Token...");
        invoke(
            &transfer_ix,
            &[
                source_pubkey.clone(),
                destination_pubkey.clone(),
                signer_pubkey.clone(),
                token_program_id.clone()
            ],
        )?;


        Ok(())
    }

    fn process_mint(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64
    ) -> ProgramResult {

        let account_info_iter = &mut accounts.iter();
        let token_program_id = next_account_info(account_info_iter)?;
        let mint_destination_account_pubkey = next_account_info(account_info_iter)?;
        let mint_token_pubkey = next_account_info(account_info_iter)?;
        let mint_signer_pubkey = next_account_info(account_info_iter)?;
        
        //wrap and mint
        if !mint_signer_pubkey.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mint_ix = spl_token::instruction::mint_to(
            token_program_id.key,
            mint_token_pubkey.key,
            mint_destination_account_pubkey.key,
            mint_signer_pubkey.key,
            &[&mint_signer_pubkey.key],
            amount
        )?;

        msg!("Minting and completing wrapping process...");
        invoke(
            &mint_ix,
            &[
                token_program_id.clone(),
                mint_token_pubkey.clone(),
                mint_destination_account_pubkey.clone(),
                mint_signer_pubkey.clone()
            ],
        )?;

        Ok(())
    }
}