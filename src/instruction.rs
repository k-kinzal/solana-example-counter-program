use solana_program::program_error::ProgramError;

/// Instructions supported by the example program.
pub enum Instruction {
    /// Increment the state
    /// 0. `[writable]` program account
    Increment,

    /// Decrement the state
    /// 0. `[writable]` program account
    Decrement,
}

impl Instruction {
    /// Unpacks a bytes into an `Instruction`.
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input {
            b"0" => Ok(Self::Increment),
            b"1" => Ok(Self::Decrement),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
