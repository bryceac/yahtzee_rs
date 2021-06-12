mod die;
mod pair;
mod combination;
mod scoreboard;
mod dialogue;
mod game_state;
mod game;

use die::Die;
use game::Game;
use game_state::GameState;
use scoreboard::Scoreboard;
use text_io::read;
use std::io::{stdout, Write};

fn main() {
    let mut game = Game::new();

    loop {
        if let GameState::GameOver = game.game_state { break }

        game.run();

        println!("Your final score is {}", game.scoreboard.total_score());
        println!("Upper section greater than or equal to 63? {}", if game.scoreboard.upper_score_is_63_or_larger() { "Yes" } else { "No" });
        println!("Multiple Yahtzees? {}", if game.scoreboard.mutliple_yahtzees() { "Yes" } else { "No" });

        let mut play_again: Option<bool> = None;

        while let None = play_again {
            print!("Would you like to play again? ");
            let _ = stdout().flush();

            let answer: String = read!();

            match answer.to_lowercase() {
                _ if answer == String::from("yes") => play_again = Some(true),
                _ if answer == String::from("no") => play_again = Some(false),
                _ => println!("Answer can only be yes or no")
            }
        }

        if play_again.unwrap() {
            game.scoreboard = Scoreboard::new();
            game.dice = [Die::new(); 5];
        } else {
            game.game_state = GameState::GameOver
        }
    }
}
