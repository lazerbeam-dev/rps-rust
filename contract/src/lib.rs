/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ env, log, near_bindgen};
use near_sdk::collections::LookupMap;

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    message: String,
    openGames: LookupMap<String, RPSGame>,
    completedGames: LookupMap<String, RPSGame>
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RPSGame {
    primary_commit: String,
    secondary_commit: Option<String>,
    state: GameState,
    winner: Option<String>
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum GameState{
    AwaitingP1,
    AwaitingP2,
    Completed,
    Abandoned
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{message: DEFAULT_MESSAGE.to_string(),
            openGames: LookupMap::new(b"m"),
            completedGames: LookupMap::new(b"m")
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;
    }

    pub fn start_game(&mut self, choice: String) {
        log!("Starting game with {}", choice);
        let gamer_id = env::signer_account_id().to_string();
        if ! self.openGames.contains_key(&gamer_id){
            let game = RPSGame{
                primary_commit: choice,
                secondary_commit: None,
                state: GameState::AwaitingP2,
                winner: None
            };
            self.openGames.insert(&gamer_id, &game);
        }
        else {
            log!("only one active game per person!");
        }
    }

    pub fn get_player_game(&mut self, gamer_id: String) -> Option<RPSGame>{
        self.openGames.get(&gamer_id)
    }

    pub fn respond(&mut self, gamer_id: String, choice: String) {
        let gamer2_id = env::signer_account_id().to_string();
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }
    
    #[test]
    fn start_game_then_get() {
        let gamer_id = env::signer_account_id().to_string();
        let mut contract = Contract::default();
        contract.start_game("rock".to_string());
        assert_eq!(contract.get_player_game(gamer_id).unwrap().primary_commit, "rock")
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
