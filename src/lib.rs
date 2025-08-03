use borsh::{BorshDeserialize,BorshSerialize};
use solana_program::{
    account_info:: {AccountInfo,next_account_info},
    entrypoint:: ProgramResult,entrypoint,
    pubkey::Pubkey,
    msg,
};

#[derive(BorshSerialize,BorshDeserialize)]

struct Counter{
    count: u32
}

enum InstructionType {
    Increment(u32),
    Decrement(u32)
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id:&Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8] // 0 1 0 1 1 1 0 1 0 1 1
)->ProgramResult{
   let acc = next_account_info(&mut accounts.iter())?;
   
   let Instruction_data = InstructionType::try_from_slice(instruction_data)?;

   // now we have instruction data so we are going to deserialize the data

   let mut converted_data = Counter::try_from_slice(&acc.data.borrow())?;

   match instruction_data {

       InstructionType::Increment(value)=> {
        msg!("Increment Value");
        converted_data.count += value
       }

        InstructionType::Decrement(value)=> {
        msg!("Decrement Value");
        converted_data.count -= value
       }
   }

   converted_data.serialize(&mut *acc.data.borrow_mut());

   msg!("Done with the contract");

}