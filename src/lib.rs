use solana_program::entrypoint ;

use processor::process_instruction;

pub mod instructions;
pub mod processor ;
pub mod state ;


#[cfg(not(feature ="no-entrypoint"))]
entrypoint!(process_instruction);