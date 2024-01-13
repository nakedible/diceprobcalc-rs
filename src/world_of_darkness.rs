use crate::{fair, Prob, IsSuccess};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Result {
    PhenomenalSuccess,
    ExceptionalSuccess,
    CompleteSuccess,
    ModerateSuccess,
    MarginalSuccess,
    Failure,
    Botch,
}

impl IsSuccess for Result {
    fn is_success(&self) -> bool {
        match self {
            Result::PhenomenalSuccess => true,
            Result::ExceptionalSuccess => true,
            Result::CompleteSuccess => true,
            Result::ModerateSuccess => true,
            Result::MarginalSuccess => true,
            _ => false,
        }
    }
}

pub enum Difficulty {
    Easy,
    Routine,
    Straightforward,
    Standard,
    Challenging,
    Difficult,
    ExtremelyDifficult,
}

impl From<Difficulty> for i64 {
    fn from(difficulty: Difficulty) -> Self {
        match difficulty {
            Difficulty::Easy => 3,
            Difficulty::Routine => 4,
            Difficulty::Straightforward => 5,
            Difficulty::Standard => 6,
            Difficulty::Challenging => 7,
            Difficulty::Difficult => 8,
            Difficulty::ExtremelyDifficult => 9,
        }
    }
}

pub fn skill_check(pool: i64, difficulty: i64) -> Prob<Result> {
    let single = |value| ((value >= difficulty) as i64, (value == 1) as i64);
    let mut out = Prob::ret((0, 0));
    for _ in 0..pool {
        out = out.map2(&fair(1, 10), |(s, b), v| {
            let (ns, nb) = single(v);
            (s + ns, b + nb)
        });
    }
    out.map1(|(s, b)| {
        if s == 0 && b > 0 {
            Result::Botch
        } else if (s - b) <= 0 {
            Result::Failure
        } else if (s - b) == 1 {
            Result::MarginalSuccess
        } else if (s - b) == 2 {
            Result::ModerateSuccess
        } else if (s - b) == 3 {
            Result::CompleteSuccess
        } else if (s - b) == 4 {
            Result::ExceptionalSuccess
        } else {
            Result::PhenomenalSuccess
        }
    })
}
