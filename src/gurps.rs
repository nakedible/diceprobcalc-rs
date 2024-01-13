use crate::{fair, Prob, IsSuccess};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Result {
    CriticalSuccess,
    Success,
    Failure,
    CriticalFailure,
}

impl IsSuccess for Result {
    fn is_success(&self) -> bool {
        match self {
            Result::CriticalSuccess => true,
            Result::Success => true,
            _ => false,
        }
    }
}

pub fn roll() -> Prob<i64> {
    let roll = fair(1, 6);
    roll.add(&roll).add(&roll)
}

pub fn skill_check(skill: i64, modifiers: i64) -> Prob<Result> {
    roll().map1(|v| {
        if v == 3 || v == 4 {
            Result::CriticalSuccess
        } else if skill + modifiers >= 15 && v == 5 {
            Result::CriticalSuccess
        } else if skill + modifiers >= 16 && v == 6 {
            Result::CriticalSuccess
        } else if v == 18 {
            Result::CriticalFailure
        } else if skill + modifiers <= 15 && v == 17 {
            Result::CriticalFailure
        } else if v == 17 {
            Result::Failure
        } else if v >= skill + modifiers + 10  {
            Result::CriticalFailure
        } else if v > skill + modifiers {
            Result::Failure
        } else {
            Result::Success
        }
    })
}
