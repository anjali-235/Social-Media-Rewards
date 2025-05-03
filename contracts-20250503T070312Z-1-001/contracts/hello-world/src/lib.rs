#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol, symbol_short, log, String};

#[contracttype]
#[derive(Clone)]
pub struct RewardEntry {
    pub user: Address,
    pub post_id: String,
    pub reward_points: u64,
}

#[contracttype]
pub enum RewardKey {
    UserPost(String), // post_id
}

const TOTAL_REWARDS: Symbol = symbol_short!("TOT_REW");

#[contract]
pub struct SocialMediaRewards;

#[contractimpl]
impl SocialMediaRewards {
    // Award reward points to a user for a specific post
    pub fn reward_post(env: Env, user: Address, post_id: String, reward_points: u64) {
        user.require_auth();

        if reward_points == 0 {
            panic!("Reward points must be greater than 0");
        }

        let entry = RewardEntry {
            user: user.clone(),
            post_id: post_id.clone(),
            reward_points,
        };

        env.storage().instance().set(&RewardKey::UserPost(post_id.clone()), &entry);

        // Update total rewards
        let mut total: u64 = env.storage().instance().get(&TOTAL_REWARDS).unwrap_or(0);
        total += reward_points;
        env.storage().instance().set(&TOTAL_REWARDS, &total);

        log!(&env, "Rewarded {} points to {} for post {}", reward_points, user, post_id);
    }

    // Get reward details for a specific post
    pub fn get_reward(env: Env, post_id: String) -> Option<RewardEntry> {
        env.storage().instance().get(&RewardKey::UserPost(post_id))
    }

    // Get total rewards given across the platform
    pub fn get_total_rewards(env: Env) -> u64 {
        env.storage().instance().get(&TOTAL_REWARDS).unwrap_or(0)
    }
}
