use count_where::CountWhere; // import library to making counting elements easier.
use crate::die::Die; // import custom type, so type will be recognized

/// Respresent the most basic pairs in a game of Yahtzee
pub enum Pair {
    ThreeOfAKind,
    FourOfAKind,
    FiveOfAKind
}

impl Pair {
    /// determine if a given roll forms any pairs
    pub fn pair(roll: &[Die; 5]) -> Option<Pair> {
        let numbers = roll.iter().map(|d| d.number).collect::<Vec<_>>();

        match numbers {
            _ if numbers.iter().filter(|n| numbers.iter().count_where(|number| number == n) == 5).peekable().peek().is_some() => Some(Pair::FiveOfAKind),
            _ if numbers.iter().filter(|n| numbers.iter().count_where(|number| number == n) == 4).peekable().peek().is_some() => Some(Pair::FourOfAKind),
            _ if numbers.iter().filter(|n| numbers.iter().count_where(|number| number == n) == 3).peekable().peek().is_some() => Some(Pair::ThreeOfAKind),
            _ => None
        }
    }
}