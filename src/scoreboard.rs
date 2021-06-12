use std::collections::HashMap;

pub struct Scoreboard {
    pub upper_section: HashMap<u32, u32>,
    pub lower_section: HashMap<String, u32>,
    pub number_of_yahtzee_bonuses: u32
}

impl Scoreboard {
    pub fn new() -> Scoreboard {
        Scoreboard {
            upper_section: HashMap::new(),
            lower_section: HashMap::new(),
            number_of_yahtzee_bonuses: 0
        }
    }

    pub fn upper_score(&self) -> u32 {
        self.upper_section.values().fold(0, |tally, value| tally + value)
    }

    pub fn lower_sscore(&self) -> u32 {
        self.lower_section.values().fold(0, |tally, value| tally + value)
    }

    pub fn upper_score_is_63_or_larger(&self) -> bool {
        self.upper_score() >= 63
    }
    
    pub fn mutliple_yahtzees(&self) -> bool {
        self.number_of_yahtzee_bonuses >= 1
    }

    pub fn total_score(&self) -> u32 {
        let mut total: u32 = 0;

        if self.upper_score_is_63_or_larger() {
            total += self.upper_score() + 35;
        } else {
            total += self.upper_score();
        }

        total += self.lower_sscore();

        if self.mutliple_yahtzees() {
            total += 100*self.number_of_yahtzee_bonuses;
        }

        return total;
    }
}