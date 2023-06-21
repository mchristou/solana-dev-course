use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum IntroInstruction {
    CreateAccount { name: String, message: String },
    UpdateAccount { name: String, message: String },
}

#[derive(BorshDeserialize)]
pub struct StudentIntro {
    name: String,
    message: String,
}

impl IntroInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        let payload = StudentIntro::try_from_slice(rest).unwrap();

        Ok(match variant {
            0 => IntroInstruction::CreateAccount {
                name: payload.name,
                message: payload.message,
            },
            1 => IntroInstruction::UpdateAccount {
                name: payload.name,
                message: payload.message,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
