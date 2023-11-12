use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program::entrypoint::ProgramResult;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use crate::instruction::Instruction;
use crate::state::State;

/// Program state handler.
pub struct Processor;

impl Processor {
    /// Processes on `Instruction`.
    /// Increment/Decrement the status of passed accounts owned by the program.
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = Instruction::unpack(input)?;
        match instruction {
            Instruction::Increment => Self::process_increment(program_id, accounts),
            Instruction::Decrement => Self::process_decrement(program_id, accounts),
        }
    }

    fn process_increment(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let account = next_account_info(account_iter)?;

        let data = &mut account.data.borrow_mut();
        let mut state = State::unpack_from_slice(&data)?;
        state.0 = state.0 + 1;
        state.pack_into_slice(data);

        Ok(())
    }

    fn process_decrement(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let account = next_account_info(account_iter)?;

        let data = &mut account.data.borrow_mut();
        let mut state = State::unpack_from_slice(&data)?;
        state.0 = state.0 - 1;
        state.pack_into_slice(data);

        Ok(())
    }
}