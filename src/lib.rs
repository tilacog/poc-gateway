#![deny(missing_docs)]

//! Axelar Gateway program for the Solana blockchain

mod entrypoint;
mod error;
pub mod instruction;
pub mod processor;

// Export current sdk types for downstream users building with a different sdk
// version
pub use solana_program;

solana_program::declare_id!("VqMMNEMXqUagHieikoHz4YgFBusPs3kMFHN59yuwaoM");
