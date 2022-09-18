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
    active_games: LookupMap<String, RPSGame>,
    completed_games: LookupMap<String, RPSGame>
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct RPSGame {
    primary_commit: String,
    secondary_commit: Option<String>,
    real_p1: Option<i8>,
    real_p2: Option<i8>,
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
            active_games: LookupMap::new(b"m"),
            completed_games: LookupMap::new(b"m")
        }
    }
}

// Implement the contract structure

// commit phase (players put hashed answer on blockchain)
// reveal phase (players give plantext answer and password, their commitments are revealed)
// resolve phase (game pays out whoever won)


#[near_bindgen]
impl Contract {
    pub fn start_game(&mut self, choice: String) -> Option<RPSGame>{
        log!("Starting game with {}", choice);
        let gamer_id = env::signer_account_id().to_string();
        if ! self.active_games.contains_key(&gamer_id){
            let game = RPSGame{
                primary_commit: choice,
                secondary_commit: None,
                real_p1: None,
                real_p2: None,
                state: GameState::AwaitingP2,
                winner: None
            };
            self.active_games.insert(&gamer_id, &game);
            Some(game)
        }
        else {
            log!("only one active game per person!");
            None
        }
    }

    pub fn get_player_game(&mut self, gamer_id: String) -> Option<RPSGame>{
        self.active_games.get(&gamer_id)
    }

    pub fn encrypt_option(option: &String, password: &String) -> String {
        "TODO: ENcrpytion".to_string()
    }

    pub fn respond(&mut self, gamer_id: &String, choice: String) {
        let gamer2_id = env::signer_account_id().to_string();
        let game = self.active_games.get(&gamer_id);
        match game {
            Some(mut g) => {
                g.secondary_commit = Some(choice);
                self.active_games.remove(&gamer_id);
                self.active_games.insert(&gamer_id, &g);
            }
            None => {
                log!("couldn't find that game");
            }
        }
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
        let game = self.active_games.get(&gamer_id);
        match game {
            Some(game) => {
                let p2_choice = Contract::choice_to_number(&game.secondary_commit.unwrap()).unwrap();
                let p1_choice = Contract::choice_to_number(&game.primary_commit).unwrap();
                Some(Contract::resolve_game(&p1_choice, &p2_choice))
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
    fn test_outcomes() {
        let rock = "rock".to_string();
        let paper = "paper".to_string();
        let scissors = "scissors".to_string();
        let rock_num = Contract::choice_to_number(&rock).unwrap();
        let scissors_num = Contract::choice_to_number(&scissors).unwrap();
        let paper_num = Contract::choice_to_number(&paper).unwrap();
        assert_eq!(Contract::resolve_game(&rock_num, &paper_num), P2Win);
        assert_eq!(Contract::resolve_game(&rock_num, &scissors_num), P1Win);
        assert_eq!(Contract::resolve_game(&scissors_num, &paper_num), P1Win);
        assert_eq!(Contract::resolve_game(&paper_num, &paper_num), Draw);
    }
    
    #[test]
    fn start_game_then_get() {
        let gamer_id = env::signer_account_id().to_string();
        let mut contract = Contract::default();
        contract.start_game("rock".to_string());
        assert_eq!(contract.get_player_game(gamer_id).unwrap().primary_commit, "rock")
    }

    #[test]
    fn start_respond_get() {
        let gamer_id = env::signer_account_id().to_string();
        let mut contract = Contract::default();
        contract.start_game("rock".to_string());
        contract.respond(&gamer_id, "paper".to_string());
        let game = contract.get_player_game(gamer_id).unwrap();
        assert_eq!(game.secondary_commit.unwrap(), "paper");
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
}
