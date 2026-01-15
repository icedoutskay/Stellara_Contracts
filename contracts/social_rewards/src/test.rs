#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_social_rewards_init() {
        let env = Env::default();
        let contract = SocialRewardsContract;
        
        let result = contract.init(&env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_record_engagement() {
        let env = Env::default();
        let contract = SocialRewardsContract;
        
        contract.init(&env).unwrap();
        
        let user = soroban_sdk::Address::random(&env);
        let engagement_type = soroban_sdk::symbol_short!("like");
        let points = 10u32;
        
        let total_points = contract
            .record_engagement(&env, user.clone(), engagement_type, points)
            .unwrap();
        
        assert_eq!(total_points, 10);
    }

    #[test]
    fn test_tier_calculation() {
        let env = Env::default();
        let contract = SocialRewardsContract;
        
        contract.init(&env).unwrap();
        
        let user = soroban_sdk::Address::random(&env);
        let engagement_type = soroban_sdk::symbol_short!("post");
        
        // Record enough engagement to reach tier 2 (100+ points)
        for _ in 0..11 {
            contract
                .record_engagement(&env, user.clone(), engagement_type, 10)
                .unwrap();
        }
        
        let rewards = contract.get_user_rewards(&env, user);
        assert!(rewards.tier >= 2);
    }

    #[test]
    fn test_claim_tier_reward() {
        let env = Env::default();
        let contract = SocialRewardsContract;
        
        contract.init(&env).unwrap();
        
        let user = soroban_sdk::Address::random(&env);
        
        // Build up points
        contract
            .record_engagement(&env, user.clone(), soroban_sdk::symbol_short!("like"), 50)
            .unwrap();
        
        let reward = contract.claim_tier_reward(&env, user).unwrap();
        assert!(reward > 0);
    }
}
