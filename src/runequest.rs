use crate::{fair, Prob, IsSuccess};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Result {
    CriticalSuccess,
    SpecialSuccess,
    Success,
    Failure,
    Fumble,
}

impl IsSuccess for Result {
    fn is_success(&self) -> bool {
        match self {
            Result::CriticalSuccess => true,
            Result::SpecialSuccess => true,
            Result::Success => true,
            _ => false,
        }
    }
}

pub fn skill_check(skill: i64, modifier: i64) -> Prob<Result> {
    fair(1, 100).map1(move |value| {
        if value == 1 || value <= (skill + modifier + 10) / 20 {
            Result::CriticalSuccess
        } else if value <= (skill + modifier + 2) / 5 {
            Result::SpecialSuccess
        } else if value == 100 || value > 100 - std::cmp::min(5, (100 - skill - modifier + 10) / 20)
        {
            Result::Fumble
        } else if value <= 5 {
            Result::Success
        } else if value >= 96 {
            Result::Failure
        } else if value <= skill + modifier {
            Result::Success
        } else {
            Result::Failure
        }
    })
}
