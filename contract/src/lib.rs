/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, log, near_bindgen, BorshStorageKey, AccountId};
use near_sdk::json_types::U64;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Records,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    records: LookupMap<AccountId, Metadata>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
    pub message: String,
    pub timestamp: U64,
}

impl Default for Contract {
  fn default() -> Self {
    Self {
      records: LookupMap::new(StorageKey::Records),
    }
  }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn set_greeting(&mut self, message: String) {
        let account_id = env::signer_account_id();
        let timestamp = env::block_timestamp();

        log!("{} set_greeting with message {} {}", account_id, message, timestamp);

        self.records.insert(&account_id, &Metadata{
            message, 
            timestamp:timestamp.into(),
        });
    }

    // `match` is similar to `switch` in other languages; here we use it to default to "Hello" if
    // self.records.get(&account_id) is not yet defined.
    // Learn more: https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
    pub fn get_greeting(&self, account_id: AccountId) -> Option::<Metadata> {
        log!("get_greeting for account_id {}", account_id);
        self.records.get(&account_id)
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn set_get_message() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Contract::default();
        contract.set_greeting("hello".to_string());
        assert_eq!(get_logs(), vec!["bob_near set_greeting with message hello 0"]);
        let context = get_context(true);
        testing_env!(context);
        assert_eq!(super::Metadata{message: "hello".into(), timestamp: 0u64.into()} ,contract.get_greeting("bob_near".parse().unwrap()).unwrap());
        assert_eq!(get_logs(), vec!["get_greeting for account_id bob_near"]);
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(true);
        testing_env!(context);
        let contract = Contract::default();
        assert_eq!(None, contract.get_greeting("francis.near".parse().unwrap()));
        assert_eq!(get_logs(), vec!["get_greeting for account_id francis.near"]);
    }
}
