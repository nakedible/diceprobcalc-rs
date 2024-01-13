use crate::{fair, Prob, IsSuccess};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StressResult {
    Nothing,
    Fumble,
    Panic,
    Trauma,
}

fn panic_roll(stress: i64) -> Prob<StressResult> {
    fair(1, 6).map1(|v| {
        if v + stress < 7 {
            StressResult::Nothing
        } else if v + stress < 10 {
            StressResult::Fumble
        } else if v + stress < 13 {
            StressResult::Panic
        } else {
            StressResult::Trauma
        }
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Trivial,
    Simple,
    Easy,
    Average,
    Demanding,
    Hard,
    Formidable,
}

impl From<Difficulty> for i64 {
    fn from(difficulty: Difficulty) -> Self {
        match difficulty {
            Difficulty::Trivial => 3,
            Difficulty::Simple => 2,
            Difficulty::Easy => 1,
            Difficulty::Average => 0,
            Difficulty::Demanding => -1,
            Difficulty::Hard => -2,
            Difficulty::Formidable => -3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Result {
    Success(i64),
    SuccessFumble(i64),
    Failure,
    FailureFumble,
    FailurePanic,
    FailureTrauma,
}

impl IsSuccess for Result {
    fn is_success(&self) -> bool {
        match self {
            Result::Success(_) => true,
            Result::SuccessFumble(_) => true,
            _ => false,
        }
    }
}

pub fn skill_check(pool: i64, stress: i64, modifier: i64) -> Prob<Result> {
    if pool + stress + modifier < 0 {
        return Prob::ret(Result::Failure);
    }
    let roll = fair(1, 6).map1(|v| (v == 1, v == 6));
    let mut ret = Prob::ret((false, 0));
    let p = pool + modifier;
    for _ in 0..p {
        ret = ret.map2(&roll, |(panic, successes), (_, s)| {
            (panic, successes + s as i64)
        });
    }
    let s = stress + std::cmp::min(0, p);
    for _ in 0..s {
        ret = ret.map2(&roll, |(panic, successes), (p, s)| {
            (panic || p, successes + s as i64)
        });
    }
    ret.bind(|(panic, successes)| {
        if panic {
            panic_roll(stress).map1(|r| match (r, successes) {
                (StressResult::Nothing, 0) => Result::Failure,
                (StressResult::Nothing, _) => Result::Success(successes),
                (StressResult::Fumble, 0) => Result::FailureFumble,
                (StressResult::Fumble, _) => Result::SuccessFumble(successes),
                (StressResult::Panic, _) => Result::FailurePanic,
                (StressResult::Trauma, _) => Result::FailureTrauma,
            })
        } else {
            Prob::ret(match successes {
                0 => Result::Failure,
                _ => Result::Success(successes),
            })
        }
    })
}

pub fn skill_check_pushed(pool: i64, stress: i64, modifier: i64) -> Prob<Result> {
    skill_check(pool, stress, modifier).bind(|result| match result {
        Result::Failure => skill_check(pool, stress + 1, modifier),
        _ => Prob::ret(result),
    })
}
