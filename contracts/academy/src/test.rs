#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_academy_contract_init() {
        let env = Env::default();
        let contract = AcademyContract;
        
        let result = contract.init(&env);
        assert!(result.is_ok());
        
        let stats = contract.get_stats(&env);
        assert_eq!(stats.total_issued, 0);
        assert_eq!(stats.next_id, 1);
    }

    #[test]
    fn test_issue_credential() {
        let env = Env::default();
        let contract = AcademyContract;
        
        contract.init(&env).unwrap();
        
        let admin = soroban_sdk::Address::random(&env);
        let holder = soroban_sdk::Address::random(&env);
        let course_id = soroban_sdk::symbol_short!("RUST");
        let level = 1u32;
        let metadata_uri = soroban_sdk::symbol_short!("ipfs");
        
        let cred_id = contract
            .issue_credential(&env, admin, holder.clone(), course_id, level, metadata_uri)
            .unwrap();
        
        assert_eq!(cred_id, 1);
        
        let stats = contract.get_stats(&env);
        assert_eq!(stats.total_issued, 1);
    }

    #[test]
    fn test_verify_credential() {
        let env = Env::default();
        let contract = AcademyContract;
        
        contract.init(&env).unwrap();
        
        let admin = soroban_sdk::Address::random(&env);
        let holder = soroban_sdk::Address::random(&env);
        let course_id = soroban_sdk::symbol_short!("JS");
        
        let cred_id = contract
            .issue_credential(
                &env,
                admin,
                holder,
                course_id,
                2,
                soroban_sdk::symbol_short!("ipfs"),
            )
            .unwrap();
        
        let exists = contract.verify_credential(&env, cred_id);
        assert!(exists);
    }
}
