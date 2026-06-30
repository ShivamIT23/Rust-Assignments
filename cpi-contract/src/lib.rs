use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;

entrypoint!(process_instruction);

fn process_instruction(
    _public_key: &Pubkey,
    accounts: &[AccountInfo],
    _transaction_data: &[u8],
) -> ProgramResult {
    let mut account_iter = accounts.iter();
    let data_account = next_account_info(&mut account_iter)?;
    let double_contract_account = next_account_info(&mut account_iter)?;

    let instructions = Instruction {
        program_id: *double_contract_account.key,
        accounts: vec![AccountMeta {
            pubkey: *data_account.key,
            is_signer: false,
            is_writable: true,
        }],
        data: vec![],
    };
    invoke(&instructions, &[data_account.clone()])?;
    Ok(())
}
