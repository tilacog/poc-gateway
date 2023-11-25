//! Instruction types

use arrayref::{array_ref, array_refs};
use solana_program::program_error::ProgramError;

const MSG_ID_SIZE: usize = 100;
const PROOF_SIZE: usize = 100;
const PAYLOAD_SIZE: usize = 100;
const QUEUE_INSTRUCTION_SIZE: usize = MSG_ID_SIZE + PROOF_SIZE + PAYLOAD_SIZE;

/// Instructions supported by the gateway program.
#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum GateweayInstruction<'a> {
    /// Receives an Axelar message and initializes a new Message account.
    ///
    /// The `Queue` instruction requires no signers and MUST be
    /// included within the same Transaction as the system program's
    /// `CreateAccount` instruction that creates the account being initialized.
    /// Otherwise another party can acquire ownership of the uninitialized
    /// account.
    ///
    /// The `Registry` account should be initialized before calling this instruction.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The messge to initialize.
    ///   1. `[]` The registry used to validate the proof.
    Queue {
        /// Messge ID
        id: &'a str,
        /// Axelar message payload
        payload: &'a [u8],
        /// Message prof
        proof: &'a [u8],
    },
}

impl<'a> GateweayInstruction<'a> {
    /// Unpacks a byte buffer into a [GatewayInstruction].
    #[allow(clippy::ptr_offset_with_cast)]
    pub fn unpack(input: &'a [u8]) -> Result<Self, ProgramError> {
        use crate::error::GatewayError::InvalidInstruction;
        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let src = array_ref![rest, 0, QUEUE_INSTRUCTION_SIZE];
                let (id, proof, payload) = array_refs![src, MSG_ID_SIZE, PROOF_SIZE, PAYLOAD_SIZE];
                let id = std::str::from_utf8(id).map_err(|_| InvalidInstruction)?;
                Self::Queue { id, payload, proof }
            }
            _ => return Err(InvalidInstruction.into()),
        })
    }
}
