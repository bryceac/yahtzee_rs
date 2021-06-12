use crate::game_state::GameState;
use crate::die::Die;
use crate::scoreboard::Scoreboard;
use crate::pair::Pair;
use crate::combination::Combination;
use crate::dialogue::Dialogue;
use std::collections::HashMap;
use maplit::hashmap;
use count_where::CountWhere;

struct Game {
    pub dice: [Die; 5],
    pub game_state: GameState,
    pub scoreboard: Scoreboard
}

impl Game {
    pub fn new() -> Game {
        Game {
            dice: [Die::new(); 5],
            game_state: GameState::NewGame,
            scoreboard: Scoreboard::new()
        }
    }

    pub fn upper_scoreboard_possibilities(&self, dice: &[Die; 5]) -> HashMap<u32, u32> {
        let numbers = dice.iter().map(|d| d.number).collect::<Vec<_>>();

        hashmap! {
            1 => ((numbers.iter().count_where(|&&n| n == 1)*1) as u32),
            2 => ((numbers.iter().count_where(|&&n| n == 2)*2) as u32),
            3 => ((numbers.iter().count_where(|&&n| n == 3)*3) as u32),
            4 => ((numbers.iter().count_where(|&&n| n == 4)*4) as u32),
            5 => ((numbers.iter().count_where(|&&n| n == 5)*5) as u32),
            6 => ((numbers.iter().count_where(|&&n| n == 6)*6) as u32),
        }
    }

    pub fn lower_scoreboard_possibilities(&self, dice: &[Die; 5]) -> HashMap<String, u32> {
        let mut possibilities: HashMap<String, u32> = hashmap! {
            String::from("Three of a Kind") => 0,
            String::from("Four of a Kind") => 0,
            String::from("Full House") => 0,
            String::from("Small Straight") => 0,
            String::from("Large Straight") => 0,
            String::from("Yahtzee") => 0,
            String::from("Chance") => 0
        };

        let sum = dice.iter().fold(0, |tally, d| tally + d.number);

        match Pair::pair(dice) {
            Some(Pair::FiveOfAKind) => {
                possibilities.insert(String::from("Three of a Kind"), sum);
                possibilities.insert(String::from("Four of a Kind"), sum);
                possibilities.insert(String::from("Yahtzee"), 50);
                possibilities.insert(String::from("Chance"), sum);
            },
            Some(Pair::FourOfAKind) => {
                possibilities.insert(String::from("Three of a Kind"), sum);
                possibilities.insert(String::from("Four of a Kind"), sum);
                possibilities.insert(String::from("Chance"), sum);
            },
            Some(Pair::ThreeOfAKind) => {
                possibilities.insert(String::from("Three of a Kind"), sum);
                possibilities.insert(String::from("Chance"), sum);
            },
            None => ()
        }

        match Combination::combination(dice) {
            Some(Combination::FullHouse) => {
                possibilities.insert(String::from("Full House"), 25);
            },
            Some(Combination::SmallStraight) => {
                possibilities.insert(String::from("Small Straight"), 30);
                possibilities.insert(String::from("Chance"), sum);
            },
            Some(Combination::LargeStraight) => {
                possibilities.insert(String::from("Large Straight"), 40);
                possibilities.insert(String::from("Small Straight"), 30);
                possibilities.insert(String::from("Chance"), sum);
            },
            None => ()
        }

        return possibilities;
    }

    fn roll(&mut self) {
        if self.dice.iter().count_where(|&&d| d.is_held) == 5 { return }
        for die in self.dice.iter_mut() {

            die.roll()
        }
    }

    fn get_numbers(&self) -> String {
        let mut results = String::new();
    
        for die in self.dice.iter() {
            let number = format!("{} ", die);
    
            results.push_str(&number);
        }
    
        return results;
    }

    fn save_die(&mut self) {
        let mut choices: Vec<String> = self.dice.iter().map(|d| format!("{}", d.number)).collect();
        choices.push(String::from("Done"));

        let dialog = Dialogue::new("Which die would you like to save (Choose Done to exit and choosing the same number again will put it back)? ", choices);
        let mut is_done = false;

        while !is_done {
            let choice = dialog.run() as usize;
            match choice {
                0 => println!("Not a valid option"),
                6 => is_done = true,
                _ => self.dice[choice-1].is_held = if self.dice[choice-1].is_held { false } else { true }
            }
        }
    }

    fn score(&mut self) {
        let upper_possibilities: HashMap<u32, u32> = self.upper_scoreboard_possibilities(&self.dice).iter().filter(|(k, _)| !self.scoreboard.upper_section.contains_key(k)).map(|(&key, &value)| (key, value)).collect();
        let lower_possibilities: HashMap<String, u32> = self.lower_scoreboard_possibilities(&self.dice).iter().filter(|(k, _)| !self.scoreboard.lower_section.contains_key(k.clone())).map(|(key, &value)| (key.clone(), value)).collect();

        let mut choices: Vec<String> = vec![];

        for (key, value) in upper_possibilities {
            choices.push(format!("{} for {} points", key, value));
        }

        for (key, value) in lower_possibilities {
            choices.push(format!("{} for {} points", key, value));
        }

        let dialog = Dialogue::new("What do you want to score? ", choices);

        loop {
            let choice = dialog.run();

            match choice {
                0 => println!("invalid choice."),
                _ => {

                }
            }
        }
    }
}

