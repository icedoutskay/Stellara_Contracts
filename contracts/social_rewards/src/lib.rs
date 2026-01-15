#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, vec, Address, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Engagement {
    pub user: Address,
    pub engagement_type: Symbol,
    pub points: u32,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct UserRewards {
    pub user: Address,
    pub total_points: u32,
    pub tier: u32,
}

#[contract]
pub struct SocialRewardsContract;

#[contractimpl]
impl SocialRewardsContract {
    /// Initialize the social rewards contract
    pub fn init(env: Env) -> Result<(), Symbol> {
        env.storage().instance().set(&symbol_short!("initialized"), &true);
        log!(&env, "Social rewards contract initialized");
        Ok(())
    }

    /// Record engagement activity
    pub fn record_engagement(
        env: Env,
        user: Address,
        engagement_type: Symbol,
        points: u32,
    ) -> Result<u32, Symbol> {
        user.require_auth();

        let engagement = Engagement {
            user: user.clone(),
            engagement_type,
            points,
            timestamp: env.ledger().timestamp(),
        };

        // Store engagement
        let engagements_key = symbol_short!("engag");
        let mut engagements: Vec<Engagement> = env
            .storage()
            .instance()
            .get(&engagements_key)
            .unwrap_or(vec![&env]);
        engagements.push_back(engagement);
        env.storage().instance().set(&engagements_key, &engagements);

        // Update user rewards
        let user_key = symbol_short!("user");
        let mut user_rewards: UserRewards = env
            .storage()
            .instance()
            .get(&user_key)
            .unwrap_or(UserRewards {
                user: user.clone(),
                total_points: 0,
                tier: 1,
            });

        user_rewards.total_points += points;
        user_rewards.tier = (user_rewards.total_points / 100) + 1;
        env.storage().instance().set(&user_key, &user_rewards);

        log!(&env, "Engagement recorded: {} points", points);
        Ok(user_rewards.total_points)
    }

    /// Get user rewards
    pub fn get_user_rewards(env: Env, user: Address) -> UserRewards {
        let user_key = symbol_short!("user");
        env.storage()
            .instance()
            .get(&user_key)
            .unwrap_or(UserRewards {
                user,
                total_points: 0,
                tier: 1,
            })
    }

    /// Get user engagement history
    pub fn get_engagement_history(env: Env, user: Address, limit: u32) -> Vec<Engagement> {
        let engagements: Vec<Engagement> = env
            .storage()
            .instance()
            .get(&symbol_short!("engag"))
            .unwrap_or(vec![&env]);

        let mut user_engagements = vec![&env];
        let mut count = 0;
        for i in (0..engagements.len()).rev() {
            if count >= limit {
                break;
            }
            if engagements.get_unchecked(i).user == user {
                user_engagements.push_back(engagements.get_unchecked(i));
                count += 1;
            }
        }
        user_engagements
    }

    /// Claim tier rewards
    pub fn claim_tier_reward(env: Env, user: Address) -> Result<u32, Symbol> {
        user.require_auth();

        let user_key = symbol_short!("user");
        let mut user_rewards: UserRewards = env
            .storage()
            .instance()
            .get(&user_key)
            .ok_or(symbol_short!("notfnd"))?;

        let reward_amount = user_rewards.tier * 10;
        user_rewards.total_points = user_rewards.total_points.saturating_sub(reward_amount);

        env.storage().instance().set(&user_key, &user_rewards);
        log!(&env, "Reward claimed: {}", reward_amount);
        Ok(reward_amount)
    }
}
