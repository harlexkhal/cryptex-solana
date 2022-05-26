
  
use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::CryptexError::InvalidInstruction;

pub enum CryptexInstruction {
  
    Stake {
        amount: u64,
    },
    
    Mint {
        amount: u64,
    },
}

impl CryptexInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::Stake {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Mint {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}