use borsh::{BorshDeserialize, BorshSerialize} ;// borsh library used to serialize and deserialize instruction data and accounts 

use solana_program::{  // solana program helper functions 
  account_info::AccountInfo, //This is a struct type that represents the format of an account on chain
  entrypoint::ProgramResult , //this is a custom type which is a Result which can either be an empty turple  for successful exit and ProgramError type for unsuccessful exit 
  program_error::ProgramError, //this is an enum variant that  enumerates different errors that a program can return , you can also add custom error variants serialized as u32's 

  pubkey::Pubkey,
};

use crate::instructions ; //reduce the path to the instructions file 
use crate::state::AddressInfo ; // reduce the path to the instructions file 


//the function called by entrypoint macro
pub fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {

    //deserialize the instruction data into AddressInfo type 
    //the function try_from_slice returns an Result type which is Ok() or Err 
    match AddressInfo::try_from_slice(&instruction_data) {
      //if the result is Ok it contains an address_info which is used to call create_address_info
       Ok(address_info) => return instructions::create::create_Address_info(
           program_id, account, address_info
       ),
       Err(_) => {},
    };

    Err(ProgramError::InvalidInstructionData)
}

