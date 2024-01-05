use std::io::Cursor;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};
use solana_program::sysvar::Sysvar;

// Define opcodes for your instructions
const CREATE_PDA_OPCODE: u8 = 0;
const INCREMENT_COUNTER_OPCODE: u8 = 1;

entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.get(0) {
        Some(&CREATE_PDA_OPCODE) => create_pda_account(program_id, accounts),
        Some(&INCREMENT_COUNTER_OPCODE) => increment_counter(program_id, accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn create_pda_account(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("create_pda_account");
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    // Ensure the user's account is a signer
    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[&user_account.key.to_bytes()[..32]],
        program_id,
    );

    if pda_account.key != &pda {
        return Err(ProgramError::InvalidArgument);
    }

    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(8); // Assuming 8 bytes for the counter

    let create_account_instruction = system_instruction::create_account(
        user_account.key,
        pda_account.key,
        lamports,
        8, // 8 bytes for the counter
        program_id,
    );

    invoke_signed(
        &create_account_instruction,
        &[user_account.clone(), pda_account.clone()],
        &[&[&user_account.key.to_bytes()[..32], &[bump_seed]]],
    )
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Count(u64);

fn increment_counter(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("increment_counter");

    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    // Ensure the user's account is a signer
    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[&user_account.key.to_bytes()[..32]],
        program_id,
    );

    if pda_account.key != &pda {
        return Err(ProgramError::InvalidArgument);
    }

    let mut data = pda_account.data.borrow_mut();
    // Try to deserialize the existing data
    let mut count = Count::try_from_slice(&data).or_else(|_| Ok::<Count, ProgramError>(Count(0)))?;
    msg!("count: {:?}", count);

    counter.0 = counter.0.checked_add(1).ok_or(ProgramError::InvalidAccountData)?;
    // count.0 = count.0 + 1;
    
    let mut cursor = Cursor::new(&mut data[..]);
    count.serialize(&mut cursor)?;


    msg!("count: {:?}", count);

    Ok(())
}