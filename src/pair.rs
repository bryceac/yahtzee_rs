use count_where::CountWhere;
use crate::die::Die;

pub enum Pair {
    ThreeOfAKind,
    FourOfAKind,
    FiveOfAKind
}

impl Pair {
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