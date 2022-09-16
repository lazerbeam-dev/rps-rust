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

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct RPSGame {
    primary_commit: String,
    secondary_commit: Option<String>,
    state: GameState,
    winner: Option<String>
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum GameState{
    AwaitingP1,
    AwaitingP2,
    Completed(GameOutcome),
    Abandoned
}

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum GameOutcome{
    P1Win, P2Win, Draw
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

    pub fn start_game(&mut self, choice: String) -> Option<RPSGame>{
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
            Some(game)
        }
        else {
            log!("only one active game per person!");
            None
        }
    }

    pub fn get_player_game(&mut self, gamer_id: String) -> Option<RPSGame>{
        self.openGames.get(&gamer_id)
    }

    pub fn respond(&mut self, gamer_id: String, choice: String) {
        let gamer2_id = env::signer_account_id().to_string();
    }

    pub fn resolve_game(option1: &i8, option2: &i8) -> GameOutcome{
        // if same, draw
        if option1 == option2 {
            GameOutcome::Draw
        }
        // if numbers are consecutive, highest wins
        else if (option1 - option2).abs() == 1 {
            if option1 > option2 { GameOutcome::P1Win } else { GameOutcome::P2Win }
        }
        // if numbers are not consecutive, lowest wins (basically just paper beating rock)
        else {
            if option1 < option2 { GameOutcome::P1Win } else { GameOutcome::P2Win }
        }
    }

    pub fn choice_to_number(choice: &String) -> Option<i8> {
        if choice == "rock" {
            Some(3)
        }
        else if choice == "scissors" {
            Some(2)
        }
        else if choice == "paper"{
            Some(1)
        }
        else {
            None
        }
    }

    pub fn resolve(&mut self, gamer_id: String) -> Option<GameOutcome>{
        let game = self.openGames.get(&gamer_id);
        match game {
            Some(game) => {
                let p2Choice = Contract::choice_to_number(&game.secondary_commit.unwrap()).unwrap();
                let p1Choice = Contract::choice_to_number(&game.primary_commit).unwrap();
                Some(Contract::resolve_game(&p1Choice, &p2Choice))
            }
            None => {
                None
            }
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::GameOutcome::{Draw, P1Win, P2Win};


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
    fn test_outcomes() {
        let contract = Contract::default();
        let rock = "rock".to_string();
        let paper = "paper".to_string();
        let scissors = "scissors".to_string();
        let rockNum = Contract::choice_to_number(&rock).unwrap();
        let scissorsNum = Contract::choice_to_number(&scissors).unwrap();
        let paperNum = Contract::choice_to_number(&paper).unwrap();
        assert_eq!(Contract::resolve_game(&rockNum, &paperNum), P2Win);
        assert_eq!(Contract::resolve_game(&rockNum, &scissorsNum), P1Win);
        assert_eq!(Contract::resolve_game(&scissorsNum, &paperNum), P1Win);
        assert_eq!(Contract::resolve_game(&paperNum, &paperNum), Draw);
    }
    
    #[test]
    fn start_game_then_get() {
        let gamer_id = env::signer_account_id().to_string();
        let mut contract = Contract::default();
        contract.start_game("rock".to_string());
        assert_eq!(contract.get_player_game(gamer_id).unwrap().primary_commit, "rock")
    }

    #[test]
    fn cant_start_two_games(){
        let gamer_id = env::signer_account_id().to_string();
        let mut contract = Contract::default();
        contract.start_game("rock".to_string());
        let second_attempt = contract.start_game("scissors".to_string());
        assert_eq!(second_attempt.is_none(), true);
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
