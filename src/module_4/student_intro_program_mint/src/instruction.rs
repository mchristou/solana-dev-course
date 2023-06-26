use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum IntroInstruction {
    CreateAccount {
        name: String,
        message: String,
    },
    UpdateAccount {
        name: String,
        message: String,
    },
    Reply {
        reply: String,
    },
    InitializeMint,
}

#[derive(BorshDeserialize)]
pub struct StudentIntro {
    name: String,
    message: String,
}

#[derive(BorshDeserialize)]
struct ReplyPayload {
    reply: String,
}

impl IntroInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (variant, rest) =
            input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let payload = StudentIntro::try_from_slice(rest).unwrap();

                IntroInstruction::CreateAccount {
                    name: payload.name,
                    message: payload.message,
                }
            }
            1 => {
                let payload = StudentIntro::try_from_slice(rest).unwrap();
                IntroInstruction::UpdateAccount {
                    name: payload.name,
                    message: payload.message,
                }
            }
            2 => {
                let payload = ReplyPayload::try_from_slice(rest).unwrap();
                IntroInstruction::Reply {
                    reply: payload.reply,
                }
            }
            3 => Self::InitializeMint,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
