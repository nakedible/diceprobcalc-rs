
use std::collections::BTreeMap;
use num_rational::BigRational;

fn fair(min: i64, max: i64) -> impl IntoIterator<Item = (i64, BigRational)> {
    let count = max - min + 1;
    (min..=max).map(move |value| (value, BigRational::new(1.into(), count.into())))
}

fn map<I: Ord, O: Ord, F: Fn(I) -> FO, FO: IntoIterator<Item = (O, BigRational)>>(a: impl IntoIterator<Item = (I, BigRational)>, f: F) -> impl IntoIterator<Item = (O, BigRational)> {
    let mut ret: BTreeMap<O, BigRational> = Default::default();
    a.into_iter().for_each(|(ak, ac)| f(ak).into_iter().for_each(|(bk, bc)| *ret.entry(bk).or_default() += bc * &ac));
    ret
}

fn print<I: IntoIterator<Item = (i64, BigRational)>>(a: I) {
    a.into_iter().for_each(|(k, c)| println!("{}: {}", k, c));
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ret = map(fair(1, 6), |x| map(fair(1, 6), move |y| Some((x + y, BigRational::new(1.into(), 1.into())))));
        print(ret);
        let result = add(2, 2);
        assert_eq!(result, 3);
    }
}
