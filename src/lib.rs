use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::collections::Map;
use near_bindgen::{near_bindgen};
use serde::{Deserialize, Serialize};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type AccountId = String;
type RoyaltyHash = Vec<u8>;

#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct Royalty {
    artists: Vec<AccountId>,    //版权的艺术家集
    shares: Vec<f64>,           //版权的分成比
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Soundario {
    royalties: Map<RoyaltyHash, Royalty>,
}

#[near_bindgen]
impl Soundario {
    pub fn new_royalty(&mut self, royalty_hash: RoyaltyHash, artists: Vec<AccountId>, shares: Vec<f64>) {
        let royalty = Royalty { artists: artists, shares: shares };
        self.royalties.insert(&royalty_hash, &royalty);
    }

    pub fn get_royalty(&self, royalty_hash: RoyaltyHash) -> Royalty {
        self.royalties.get(&royalty_hash).unwrap_or_default()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_bindgen::MockedBlockchain;
    use near_bindgen::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn set_get_royalty() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Soundario::default();
        let artists = vec!["zhao".to_string(), "qian".to_string(), "sun".to_string(), "li".to_string()];
        let shares = vec![10.0, 20.0, 30.0, 40.0];
        let royalty_hash = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let artists2 = vec!["zhou".to_string(), "wu".to_string(), "zheng".to_string(), "wang".to_string()];
        let shares2 = vec![40.0, 30.0, 20.0, 10.0];
        let royalty_hash2 = vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40];
        contract.new_royalty(royalty_hash.clone(), artists.clone(), shares.clone());
        contract.new_royalty(royalty_hash2.clone(), artists2.clone(), shares2.clone());
        let royalty = contract.get_royalty(royalty_hash);
        assert_eq!(artists, royalty.artists);
        assert_eq!(shares, royalty.shares);
        let royalty2 = contract.get_royalty(royalty_hash2);
        assert_eq!(artists2, royalty2.artists);
        assert_eq!(shares2, royalty2.shares);
    }
}

