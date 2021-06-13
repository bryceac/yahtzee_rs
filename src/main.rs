mod game;
mod dialogue;

// import stuff needed from root directory.
use game::Game;

// import stuff from internal library, to make sure things are recognized.
use lib_yahtzee::{ Die, GameState, Scoreboard };

// import stuff needed to deal with console
use text_io::read;
use std::io::{stdout, Write};

fn main() {

    // create game object, to be able to run and modify game easily.
    let mut game = Game::new();

    // run a loop that will continue until the game state is game over, to ensure player can play as long as they want.
    loop {
        if let GameState::GameOver = game.game_state { break }

        game.run();

        // retrieve score and notify users of bonuses, so they know how well they did.
        println!("Your final score is {}", game.scoreboard.total_score());
        println!("Upper section greater than or equal to 63? {}", if game.scoreboard.upper_score_is_63_or_larger() { "Yes" } else { "No" });
        println!("Multiple Yahtzees? {}", if game.scoreboard.mutliple_yahtzees() { "Yes" } else { "No" });

        // allow the user the chance to immediately start a new round.
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