use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

/// Structure representing a counter.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    /// Current value of the counter.
    pub count: u32,
}

entrypoint!(process_instruction);

/// Instruction handler for the smart contract.
///
/// # Arguments
///
/// * `program_id` - Pubkey of the program.
/// * `accounts` - List of accounts involved in the instruction.
/// * `instruction_data` - Data of the instruction.
///
/// # Returns
///
/// Returns `ProgramResult` on success or `ProgramError` on failure.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Initialize the iterator over accounts.
    let accounts_iter = &mut accounts.iter();

    // Get the user account.
    let user = next_account_info(accounts_iter)?;
    // Get the counter account.
    let counter_account = next_account_info(accounts_iter)?;
    // Get the system program account.
    let system_program = next_account_info(accounts_iter)?;

    // Find the program-derived address (PDA) and bump seed for the counter account.
    let (counter_pda, bump_seed) = Pubkey::find_program_address(&[user.key.as_ref()], program_id);
    if counter_pda != *counter_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the user is the owner of the counter account.
    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Match on the first byte of the instruction data to determine the action.
    match instruction_data[0] {
        // Initialize the counter account.
        0 => {
            msg!("Initialize counter...");
            let rent = Rent::get()?;
            let required_lamports = rent.minimum_balance(std::mem::size_of::<Counter>());

            invoke_signed(
                &system_instruction::create_account(
                    user.key,
                    counter_account.key,
                    required_lamports,
                    std::mem::size_of::<Counter>() as u64,
                    program_id,
                ),
                &[
                    user.clone(),
                    counter_account.clone(),
                    system_program.clone(),
                ],
                &[&[user.key.as_ref(), &[bump_seed]]],
            )?;

            let mut counter = Counter::try_from_slice(&counter_account.data.borrow())?;
            counter.count = 0;
            counter.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;
            msg!("Counter initialized!");
        }
        // Increment the counter value.
        1 => {
            msg!("Increment counter...");
            let mut counter = Counter::try_from_slice(&counter_account.data.borrow())?;
            counter.count += 1;
            counter.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;
            msg!("Counter updated!");
        }
        // Get the current counter value.
        2 => {
            msg!("Get counter value...");
            let counter = Counter::try_from_slice(&counter_account.data.borrow())?;
            msg!("Counter value: {}", counter.count);
        }
        // Handle invalid instruction data.
        _ => {
            msg!("Invalid instruction");
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}
