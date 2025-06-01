#[cfg(test)]
mod tests {
    use mollusk_svm::{result::Check, Mollusk};
    use solana_sdk::account::Account;
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::signature::Keypair;
    use solana_sdk::signer::Signer;

    fn setup_test_context() -> (Mollusk, Pubkey, Account) {
        let program_id = Keypair::new().pubkey();

        let mollusk = Mollusk::new(&program_id, "deploy/log");

        let test_account = Account {
            data: vec![],
            lamports: LAMPORTS_PER_SOL,
            owner: program_id,
            rent_epoch: u64::MAX,
            ..Default::default()
        };

        (mollusk, program_id, test_account)
    }

    #[test]
    fn test_hello_solana_logs_message() {
        let ( mollusk, program_id, test_account) = setup_test_context();
        let test_account_pk = Keypair::new().pubkey();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[], // no ix data
            vec![
                AccountMeta::new(test_account_pk, true),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &instruction,
            &[(test_account_pk, test_account)],
            &[
                Check::success(),

                Check::compute_units(105) // why is it 105 syscall = 100
            ]
        );

        match &result.program_result {
            mollusk_svm::result::ProgramResult::Success => {
                println!("Program executed successfully and logged message");
            }
            mollusk_svm::result::ProgramResult::Failure(err) => {
                panic!("Program failed: {:?}", err);
            }
            mollusk_svm::result::ProgramResult::UnknownError(err) => {
                panic!("Program encountered unknown error: {:?}", err);
            }
        }
    }
}