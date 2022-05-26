use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
};

use crate::processor::Processor;

entrypoint!(process_instruction);
fn process_instruction(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(accounts, instruction_data)
}