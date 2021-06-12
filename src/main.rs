mod die;
mod pair;
mod combination;
mod scoreboard;
mod dialogue;
mod game_state;
mod game;

use die::Die;
use dialogue::Dialogue;

fn main() {
    let mut dice = [Die::new(); 5];
    
}

fn get_numbers(dice: &[Die; 5]) -> String {
    let mut results = String::new();

    for die in dice {
        let number = format!("{} ", die);

        results.push_str(&number);
    }

    return results;
}
