use maplit::hashmap;
use std::collections::HashMap;

pub struct Scoreboard {
    pub upper_section: HashMap<u32, u32>,
    pub lower_section: HashMap<String, u32>,
    pub number_of_yahtzee_bonuses: u32
}

impl Scoreboard {
    pub fn new() -> Scoreboard {
        Scoreboard {
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

    pub fn upper_score(&self) -> u32 {
        self.upper_section.values().fold(0, |tally, value| tally + value)
    }

    pub fn lower_sscore(&self) -> u32 {
        self.lower_section.values().fold(0, |tally, value| tally + value)
    }

    pub fn total_score(&self) -> u32 {
        let mut total: u32 = 0;

        if self.upper_score() >= 63 {
            total += self.upper_score() + 35;
        } else {
            total += self.upper_score();
        }

        total += self.lower_sscore();

        if self.number_of_yahtzee_bonuses > 0 {
            total += 100*self.number_of_yahtzee_bonuses;
        }

        return total;
    }
}