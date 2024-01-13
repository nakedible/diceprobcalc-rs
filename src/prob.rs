use num::rational::BigRational;
use num::ToPrimitive;
use std::collections::{BTreeMap, BTreeSet};

pub struct Prob<V: Ord + Clone + std::fmt::Debug>(BTreeMap<V, BigRational>);

impl<V: Ord + Clone + std::fmt::Debug> Prob<V> {
    pub fn ret(v: V) -> Self {
        Self(
            [(v, BigRational::new(1.into(), 1.into()))].into()
        )
    }

    pub fn gen(&self) -> impl Iterator<Item = (V, BigRational)> + '_ {
        self.0.iter().map(|(k, v)| (k.clone(), v.clone()))
    }

    pub fn bind<BV: Ord + Clone + std::fmt::Debug>(&self, f: impl Fn(V) -> Prob<BV>) -> Prob<BV> {
        let mut ret = BTreeMap::new();
        for (ak, ac) in &self.0 {
            for (bk, bc) in f(ak.clone()).0 {
                *ret.entry(bk).or_default() += bc * ac;
            }
        }
        Prob(ret)
    }

    pub fn map1<BV: Ord + Clone + std::fmt::Debug>(&self, f: impl Fn(V) -> BV) -> Prob<BV> {
        self.bind(move |v| Prob::ret(f(v)))
    }

    pub fn map2<B: Ord + Clone + std::fmt::Debug, C: Ord + Clone + std::fmt::Debug>(
        &self,
        b: &Prob<B>,
        f: impl Fn(V, B) -> C,
    ) -> Prob<C> {
        self.bind(|a| b.map1(|b| f(a.clone(), b)))
    }

    pub fn print(&self) {
        let mut cumu = BigRational::new(0.into(), 1.into());
        let mut cumd = BigRational::new(1.into(), 1.into());
        for (k, c) in self.0.iter() {
            cumu += c;
            println!(
                "{:20}: {:.3} {:20} | {:.3} {:20} | {:.3} {:20}",
                format!("{:?}", k),
                c.to_f64().unwrap(),
                c,
                cumu.to_f64().unwrap(),
                cumu,
                cumd.to_f64().unwrap(),
                cumd
            );
            cumd -= c;
        }
    }
}

pub fn print_range<V: Clone + std::fmt::Debug, BV: Ord + Clone + std::fmt::Debug>(
    range: impl IntoIterator<Item = V>,
    f: impl Fn(V) -> Prob<BV>,
) {
    let mut cols = BTreeSet::new();
    let output = range
        .into_iter()
        .map(|v| (v.clone(), f(v)))
        .collect::<Vec<_>>();
    output.iter().for_each(|(_, p)| {
        for (k, _) in p.gen() {
            cols.insert(k);
        }
    });
    print!("{:20}", "i");
    for k in &cols {
        print!(" | {:20}", format!("{:?}", k));
    }
    println!();
    for (i, p) in output {
        print!("{:20}", format!("{:?}", i));
        for k in &cols {
            print!(
                " | {:20.2}",
                p.0.get(k).and_then(|v| v.to_f64()).unwrap_or(0.0)
            );
        }
        println!();
    }
}

impl<V: Ord + Clone + std::fmt::Debug + std::ops::Add> Prob<V>
where
    <V as std::ops::Add>::Output: Ord + Clone + std::fmt::Debug,
{
    pub fn add(&self, other: &Self) -> Prob<<V as std::ops::Add>::Output> {
        self.map2(other, |a, b| a + b)
    }
}

impl<V: Ord + Clone + std::fmt::Debug + std::ops::Sub> Prob<V>
where
    <V as std::ops::Sub>::Output: Ord + Clone + std::fmt::Debug,
{
    pub fn sub(&self, other: &Self) -> Prob<<V as std::ops::Sub>::Output> {
        self.map2(other, |a, b| a - b)
    }
}

pub fn fair(min: i64, max: i64) -> Prob<i64> {
    let count = max - min + 1;
    Prob(
        (min..=max)
            .map(move |value| (value, BigRational::new(1.into(), count.into())))
            .collect(),
    )
}
