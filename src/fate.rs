use crate::{fair, Prob, IsSuccess};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Result {
    SuccessWithStyle,
    Success,
    Tie,
    Failure,
}

impl IsSuccess for Result {
    fn is_success(&self) -> bool {
        match self {
            Result::SuccessWithStyle => true,
            Result::Success => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Terrible,
    Poor,
    Mediocre,
    Average,
    Fair,
    Good,
    Great,
    Superb,
    Fantastic,
    Epic,
    Legendary,
}

impl From<Difficulty> for i64 {
    fn from(difficulty: Difficulty) -> Self {
        match difficulty {
            Difficulty::Terrible => -2,
            Difficulty::Poor => -1,
            Difficulty::Mediocre => 0,
            Difficulty::Average => 1,
            Difficulty::Fair => 2,
            Difficulty::Good => 3,
            Difficulty::Great => 4,
            Difficulty::Superb => 5,
            Difficulty::Fantastic => 6,
            Difficulty::Epic => 7,
            Difficulty::Legendary => 8,
        }
    }
}

pub fn roll() -> Prob<i64> {
    let roll = fair(-1, 1);
    roll.add(&roll).add(&roll).add(&roll)
}

pub fn skill_check(skill: i64, opposition: i64) -> Prob<Result> {
    roll().map1(|v| {
        if v + skill > opposition + 3 {
            Result::SuccessWithStyle
        } else if v + skill > opposition {
            Result::Success
        } else if v + skill == opposition {
            Result::Tie
        } else {
            Result::Failure
        }
    })
}
