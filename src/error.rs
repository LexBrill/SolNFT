use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum NFTError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<NFTError> for ProgramError {
    fn from (e: NFTError) -> Self {
        ProgramError::Custom(e as u32)
    }
}