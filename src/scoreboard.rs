use maplit::hashmap;
use std::collections::HashMap;

struct ScoreBoard {
    upper_section: HashMap<u32, u32>,
    lower_section: HashMap<String, u32>,
    number_of_yahtzee_bonuses: u32
}

impl ScoreBoard {
    fn new() -> ScoreBoard {
        ScoreBoard {
            upper_section: hashmap![
                1 => 0,
                2 => 0,
                3 => 0,
                4 => 0,
                5 => 0,
                6 => 0
            ],
            lower_section: hashmap![
                String::from("Three of a Kind") => 0,
                String::from("Four of a Kind") => 0,
                String::from("Full House") => 0,
                String::from("Small Straight") => 0,
                String::from("Large Straight") => 0,
                String::from("Yahtzee") => 0,
                String::from("Chance") => 0
            ],
            number_of_yahtzee_bonuses: 0
        }
    }
}