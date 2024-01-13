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
