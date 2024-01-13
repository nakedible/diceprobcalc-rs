use crate::{fair, Prob, IsSuccess};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Result {
    CriticalSuccess,
    Success,
    Failure,
    Fumble,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Regular,
    Hard,
    Extreme,
}

pub fn skill_check(
    skill: i64,
    difficulty: Difficulty,
) -> Prob<Result> {
    fair(1, 100).bind(move |value| {
        let target = match difficulty {
            Difficulty::Regular => skill,
            Difficulty::Hard => skill / 2,
            Difficulty::Extreme => skill / 5,
        };
        let ret = if value == 1 {
            Result::CriticalSuccess
        } else if value == 100 {
            Result::Fumble
        } else if target < 50 && value >= 96 {
            Result::Fumble
        } else if value <= target {
            Result::Success
        } else {
            Result::Failure
        };
        Prob::ret(ret)
    })
}

pub fn skill_check_pushed(
    skill: i64,
    difficulty: Difficulty,
) -> Prob<Result> {
    skill_check(skill, difficulty).bind(|result| match result {
        Result::Failure => skill_check(skill, difficulty),
        _ => Prob::ret(result),
    })
}
