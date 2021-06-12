use crate::game_state::GameState;
use crate::die::Die;
use crate::scoreboard::Scoreboard;
use crate::pair::Pair;
use crate::combination::Combination;
use crate::dialogue::Dialogue;
use std::{ collections::HashMap, process };
use maplit::hashmap;
use count_where::CountWhere;
use regex::Regex;

pub struct Game {
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

    pub fn upper_scoreboard_possibilities(&self) -> HashMap<u32, u32> {
        let numbers = self.dice.iter().map(|d| d.number).collect::<Vec<_>>();

        hashmap! {
            1 => ((numbers.iter().count_where(|&&n| n == 1)*1) as u32),
            2 => ((numbers.iter().count_where(|&&n| n == 2)*2) as u32),
            3 => ((numbers.iter().count_where(|&&n| n == 3)*3) as u32),
            4 => ((numbers.iter().count_where(|&&n| n == 4)*4) as u32),
            5 => ((numbers.iter().count_where(|&&n| n == 5)*5) as u32),
            6 => ((numbers.iter().count_where(|&&n| n == 6)*6) as u32),
        }
    }

    pub fn lower_scoreboard_possibilities(&self) -> HashMap<String, u32> {
        let mut possibilities: HashMap<String, u32> = hashmap! {
            String::from("Three of a Kind") => 0,
            String::from("Four of a Kind") => 0,
            String::from("Full House") => 0,
            String::from("Small Straight") => 0,
            String::from("Large Straight") => 0,
            String::from("Yahtzee") => 0,
            String::from("Chance") => 0
        };

        let sum = self.dice.iter().fold(0, |tally, d| tally + d.number);

        match Pair::pair(&self.dice) {
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
            None => {
                possibilities.insert(String::from("Chance"), sum);
            }
        }

        match Combination::combination(&self.dice) {
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

    pub fn run(&mut self) {
        while !self.game_is_complete() {
            self.display_upper_section();
            self.display_lower_section();

            self.play_turn();
        }
    }

    fn display_upper_section(&self) {
        if self.scoreboard.upper_section.is_empty() { return }

        for (key, number) in self.scoreboard.upper_section.clone() {
            println!("{}: {}", key, number)
        }
        println!("-----");
    }

    fn display_lower_section(&self) {
        if self.scoreboard.lower_section.is_empty() { return }

        for (key, number) in self.scoreboard.lower_section.clone() {
            println!("{}: {}", key, number)
        }
        println!("-----");
    }

    fn filled_fields(&self) -> u32 {
        let total = self.scoreboard.upper_section.len() + self.scoreboard.lower_section.len();

        return total as u32;
    }

    fn game_is_complete(&self) -> bool {
        self.filled_fields() == 13
    }

    fn roll(&mut self) -> Result<(), String> {
        if self.dice.iter().count_where(|&&d| d.is_held) == 5 { 
            return Err(String::from("There are no dice to roll."));
         }
        for die in self.dice.iter_mut() {

            if !die.is_held {
                die.roll()
            }
            
        }
        Ok(())
    }

    fn get_numbers(&self) -> String {
        if self.dice.iter().count_where(|&&d| d.number == 0) == 5 { return String::new(); }
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
                _ => {
                    self.dice[choice-1].is_held = if self.dice[choice-1].is_held { false } else { true };
                    
                    if self.dice[choice-1].is_held {
                        println!("saved die {}", choice);
                    } else {
                        println!("put back die {}", choice);
                    }
                }
            }
        }
    }

    fn score(&mut self) {
        let upper_possibilities: HashMap<u32, u32> = self.upper_scoreboard_possibilities().iter().filter(|(k, _)| !self.scoreboard.upper_section.contains_key(k)).map(|(&key, &value)| (key, value)).collect();
        let lower_possibilities: HashMap<String, u32> = self.lower_scoreboard_possibilities().iter().filter(|(k, _)| !self.scoreboard.lower_section.contains_key(k.clone())).map(|(key, &value)| (key.clone(), value)).collect();

        let mut choices: Vec<String> = vec![];

        for (key, value) in upper_possibilities {
            choices.push(format!("{} for {} points", key, value));
        }

        for (key, value) in lower_possibilities {
            choices.push(format!("{} for {} points", key, value));
        }

        let dialog = Dialogue::new("What do you want to score? ", choices);

        loop {
            let choice = dialog.run() as usize;

            match choice {
                0 => println!("invalid choice."),
                _ => {
                    let selection = dialog.choices[choice-1].clone();

                    if let Some(key) = key_search(&selection) {
                        if let Ok(number) = key.parse::<u32>() {
                            if let Some(value) = self.upper_scoreboard_possibilities().get(&number) {
                                self.scoreboard.upper_section.insert(number, *value);
                            }
                        } else {
                            if let Some(value) = self.lower_scoreboard_possibilities().get(&key) {
                                self.scoreboard.lower_section.insert(key, *value);
                            }
                        }
                    }
                    break;
                }
            }
        }
    }

    fn play_turn(&mut self) {
        let mut rolls = 0;

        if rolls == 0 {
            for die in self.dice.iter_mut() {
                if die.is_held {
                    die.is_held = false
                }
            }
        }



        while rolls <= 3 {
            let roll = self.get_numbers();

            println!("{}", roll);

            match rolls {
                0 => {
                    let dialog = Dialogue::new("What would you like to do? ", vec![String::from("Roll"), String::from("quit")]);

                    let choice = dialog.run();

                    match choice {
                        1 => {
                            if let Err(e) = self.roll() {
                                println!("{}", e)
                            } else {
                                rolls += 1
                            }    
                        },
                        2 => {

                            if cfg!(windows) {
                                process::exit(256);
                            } else {
                                process::exit(0);
                            }  
                        },
                        _ => println!("Invalid choice.")
                    }
                },
                1..=2 => {
                    let dialog = Dialogue::new("What would you like to do? ", vec![String::from("Hold Dice"), String::from("Roll"), String::from("Score"), String::from("Quit")]);

                    let choice = dialog.run();

                    match choice {
                        1 => self.save_die(),
                        2 => {
                            {
                                if let Err(e) = self.roll() {
                                    println!("{}", e)
                                } else {
                                    rolls += 1
                                }    
                            }
                        },
                        3 => {
                            self.score();
                            rolls = 4;
                        },
                        4 => {

                            if cfg!(windows) {
                                process::exit(256);
                            } else {
                                process::exit(0);
                            }  
                        },
                        _ => println!("invalid choice.")
                    }
                },
                3 => {
                    let dialog = Dialogue::new("What would you like to do? ", vec![String::from("Score"), String::from("Quit")]);

                    let choice = dialog.run();

                    match choice {
                        1 => {
                            self.score();
                            rolls = 4;
                        },
                        2 => {

                            if cfg!(windows) {
                                process::exit(256);
                            } else {
                                process::exit(0);
                            }  
                        },
                        _ => println!("Invalid choice.")
                    }
                },
                _ => ()
            }
        }
    }
}

fn key_search(text: &String) -> Option<String> {
    if let Ok(expression) = Regex::new("(.*?) for") {
        if let Some(captures) = expression.captures(text) {
            Some(String::from(&captures[1]))
        } else {
            None
        }
    } else {
        None
    }
}

