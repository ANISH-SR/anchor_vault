#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(unused)]

#[cfg(test)]
mod tests {
    use anchor_lang::prelude::*; 
    use anchor_lang::InstructionData;
    use anchor_vault::instruction;
    use mollusk_svm::{program, Mollusk, result::Check};
    use solana_sdk::{
        account::Account, 
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL, 
        pubkey,
    };

    // use {
    //     mollusk_svm_bencher::MolluskComputeUnitBencher,
    // };
    //// disable logging (optional)
    // solana_logger::setup_with("");

    const ID: pubkey::Pubkey = pubkey!("FGxVc2HAfo2bDARNMtDRzKwCbRCT8XpvBiYUYqPjLhqt");
    const USER: pubkey::Pubkey = pubkey::Pubkey::new_from_array([0x01; 32]);

    #[test]
    fn test_initialize() {
        let mollusk = Mollusk::new(&ID, "../../target/deploy/anchor_vault");

        let (state_pda, state_bump) =
            Pubkey::find_program_address(&[b"state", USER.as_ref()], &ID);
        let (vault_pda, vault_bump) =
            Pubkey::find_program_address(&[b"vault", state_pda.as_ref()], &ID);

        let (system_program, system_account) = program::keyed_account_for_system_program();

        // User: some lamports, owned by system
        let user_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        // State account: rent exempt, space = 2 bytes, owned by program
        let state_account = Account::new(
            Rent::default().minimum_balance(8 + 2), // discriminator + VaultState (2 bytes)
            8 + 2,
            &ID,
        );

        // Vault: just needs to exist and receive lamports, also owned by program
        let vault_account = Account::new(
            Rent::default().minimum_balance(0),
            0,
            &ID,
        );

        let ix_accs = vec![
            AccountMeta::new(USER, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(system_program, false),
        ];

        let data = instruction::Initialize {}.data();

        let ix = Instruction::new_with_bytes(ID, &data, ix_accs);

        let tx_accounts = vec![
            (USER, user_account),
            (state_pda, state_account),
            (vault_pda, vault_account),
            (system_program, system_account),
        ];

        mollusk.process_and_validate_instruction(&ix, &tx_accounts, &[Check::success()]);
    }

    
    #[test]
    fn test_deposit() {
        //mollusk instance
        let mollusk = Mollusk::new(&ID, "../../target/deploy/anchor_vault");

        //Pubkeys
        let (state_pda, state_bump) = 
            pubkey::Pubkey::find_program_address(&[b"state", USER.as_ref()], &ID);

        let (vault_pda, vault_bump) = 
            pubkey::Pubkey::find_program_address(&[b"vault", state_pda.as_ref()], &ID);

        let (system_program, system_account) = program::keyed_account_for_system_program();

        //Build the accounts
        let user_accounts = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);
        let state_account = Account::new(0, 0, &system_program);
        let vault_account = Account::new(0, 0, &system_program);

        //get the accounts meta
        let ix_accs = vec![
            AccountMeta::new(USER, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(system_program, false),
        ];

        //Data
        let transfer_amount: u64 = 500_000_000;
        let data = (anchor_vault::instruction::Deposit {
            amount: transfer_amount,
        })
        .data();

        //Build Ix
        let instructions = Instruction::new_with_bytes(ID, &data, ix_accs);

        //Get Tx Accounts
        let tx_accs = &vec![
            (USER, user_accounts.clone()),
            (state_pda, state_account.clone()),    //system_account.clone()
            (vault_pda, vault_account.clone()),
            (system_program, system_account.clone()),
        ];

        //process and validate our instruction
        let test_deposit = 
            mollusk.process_and_validate_instruction(&instructions, &tx_accs, &[Check::success()]);
        
    }

    #[test]
    fn test_withdraw() {
        // Example mollusk test - uncomment imports when you need them
        // use mollusk_svm::{result::Check, Mollusk};
        // use anchor_lang::{InstructionData, ToAccountMetas};
        // use solana_program::instruction::Instruction;
        
        // let program_id = anchor_vault::id();
        // println!("Program ID: {}", program_id);
        
        // Add your mollusk test logic here
    }
}