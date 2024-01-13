pub mod prob;
pub mod call_of_cthulhu;
pub mod runequest;
pub mod rolemaster;
pub mod world_of_darkness;
pub mod fate;
pub mod gurps;
pub mod alien_rpg;

pub use prob::*;

trait IsSuccess {
    fn is_success(&self) -> bool;
}

pub fn x2d6m2d6() -> Prob<i64> {
    let x1d6 = fair(1, 6);
    let x2d6 = x1d6.add(&x1d6);
    x2d6.sub(&x2d6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x1d6 = fair(1, 6);
        let x2d6 = x1d6.map2(&x1d6, |a, b| a + b);
        x2d6.map2(&x2d6, |a, b| a - b).print();
    }
}
