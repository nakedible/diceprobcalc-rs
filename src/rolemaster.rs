use crate::{fair, Prob, IsSuccess};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Result {
    AbsoluteSuccess,
    Success,
    UnusualSuccess,
    NearSuccess,
    PartialSuccess,
    UnusualEvent,
    Failure,
    AbsoluteFailure,
    SpectacularFailure,
}

impl IsSuccess for Result {
    fn is_success(&self) -> bool {
        match self {
            Result::AbsoluteSuccess => true,
            Result::Success => true,
            Result::UnusualSuccess => true,
            Result::NearSuccess => true,
            Result::PartialSuccess => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Routine,
    Easy,
    Light,
    Medium,
    Hard,
    VeryHard,
    ExtremelyHard,
    SheerFolly,
    Absurd,
}

impl From<Difficulty> for i64 {
    fn from(difficulty: Difficulty) -> Self {
        match difficulty {
            Difficulty::Routine => 30,
            Difficulty::Easy => 20,
            Difficulty::Light => 10,
            Difficulty::Medium => 0,
            Difficulty::Hard => -10,
            Difficulty::VeryHard => -20,
            Difficulty::ExtremelyHard => -30,
            Difficulty::SheerFolly => -50,
            Difficulty::Absurd => -70,
        }
    }
}

pub fn roll_oe(count: usize) -> Prob<i64> {
    fair(1, 100).bind(move |v| {
        if count > 1 && v >= 96 {
            roll_oe(count - 1).map1(|o| v + o)
        } else {
            Prob::ret(v)
        }
    })
}

pub fn roll(low: bool, high: bool) -> Prob<i64> {
    fair(1, 100).bind(move |v| {
        if low && v <= 5 {
            roll_oe(2).map1(|o| v - o)
        } else if high && v != 100 && v >= 96 {
            roll_oe(2).map1(|o| v + o)
        } else {
            Prob::ret(v)
        }
    })
}

pub fn skill_check(skill: i64, difficulty: i64) -> Prob<Result> {
    roll(true, true).map1(|v| {
        if v == 100 {
            Result::UnusualSuccess
        } else if v == 66 {
            Result::UnusualEvent
        } else if (v + skill + difficulty) <= -26 {
            Result::SpectacularFailure
        } else if (v + skill + difficulty) <= 4 {
            Result::AbsoluteFailure
        } else if (v + skill + difficulty) <= 75 {
            Result::Failure
        } else if (v + skill + difficulty) <= 90 {
            Result::PartialSuccess
        } else if (v + skill + difficulty) <= 110 {
            Result::NearSuccess
        } else if (v + skill + difficulty) <= 175 {
            Result::Success
        } else {
            Result::AbsoluteSuccess
        }
    })
}
