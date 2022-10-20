/*
 * Mock for rewards smart contract
 *
 * It should interact with staking contract getting user rewards
 *
 */

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::{U128, U64},
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserReward {
    pub reward_epoch: U64,
    pub user_veorder: U128,
    pub total_veorder: U128,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserRewards {
    pub rewards: Vec<UserReward>,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Rewards {
    stake: AccountId,
}

// Implement the contract structure
#[near_bindgen]
impl Rewards {
    /// Initializes staking contract state.
    #[init]
    pub fn new(stake: AccountId) -> Self {
        Self { stake }
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_rewards(&self) -> UserRewards {
        return UserRewards {
            rewards: vec![UserReward {
                reward_epoch: U64::from(1),
                user_veorder: U128::from(1),
                total_veorder: U128::from(1),
            }],
        };
    }
}
