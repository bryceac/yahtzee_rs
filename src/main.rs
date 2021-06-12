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
    let mut numbers: String = String::new();

    let dialog = Dialogue::new("What would you like to do? ", vec![String::from("Roll"), String::from("Quit")]);

    loop {
        if numbers.is_empty() {} else {
            println!("{}", numbers);
        }

        match dialog.run() {
            1 => {
                roll(&mut dice);
                numbers = get_numbers(&dice);
            },
            2 => break,
            _ => {}
        }
    }
}

fn get_numbers(dice: &[Die; 5]) -> String {
    let mut results = String::new();

    for die in dice {
        let number = format!("{} ", die);

        results.push_str(&number);
    }

    return results;
}
