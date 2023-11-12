use solana_program::program_error::ProgramError;
use solana_program::program_pack::{IsInitialized, Pack, Sealed};

/// State that the sample program has.
pub struct State(pub u64);

impl IsInitialized for State {
    fn is_initialized(&self) -> bool {
        self.0 != 0
    }
}

impl Sealed for State {}

impl Pack for State {
    const LEN: usize = 8;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst.copy_from_slice(&self.0.to_le_bytes());
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let bytes = src.try_into().map_err(|_| ProgramError::InvalidAccountData)?;
        let value = u64::from_le_bytes(bytes);
        Ok(Self(value))
    }
}
