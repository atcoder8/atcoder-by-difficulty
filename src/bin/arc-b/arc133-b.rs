use itertools::Itertools;
use proconio::input;

use crate::lis::StronglyLIS;

fn main() {
    input! {
        n: usize,
        pp: [usize; n],
        qq: [usize; n],
    }

    let mut inv_qq = vec![n; n + 1];
    for (i, &q) in qq.iter().enumerate() {
        inv_qq[q] = i;
    }

    let mut lis = StronglyLIS::new();
    for &p in &pp {
        let positions = (p..=n).step_by(p).map(|i| inv_qq[i]).sorted();
        for pos in positions.into_iter().rev() {
            lis.push(pos);
        }
    }

    println!("{}", lis.lis_len());
}

pub mod lis {
    use superslice::Ext;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct WeeklyLIS<T>
    where
        T: Clone + Ord,
    {
        dp: Vec<T>,
    }

    impl<T> WeeklyLIS<T>
    where
        T: Clone + Ord,
    {
        pub fn new() -> Self {
            Self { dp: vec![] }
        }

        pub fn push(&mut self, x: T) {
            let idx = self.dp.upper_bound(&x);
            if idx < self.dp.len() {
                self.dp[idx] = x;
            } else {
                self.dp.push(x);
            }
        }

        pub fn lis_len(&self) -> usize {
            self.dp.len()
        }
    }

    impl<T> Default for WeeklyLIS<T>
    where
        T: Clone + Ord,
    {
        fn default() -> Self {
            WeeklyLIS::new()
        }
    }

    impl<T> From<Vec<T>> for WeeklyLIS<T>
    where
        T: Clone + Ord,
    {
        fn from(seq: Vec<T>) -> Self {
            let mut lis = Self::default();
            for x in seq {
                lis.push(x);
            }
            lis
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct StronglyLIS<T>
    where
        T: Clone + Ord,
    {
        dp: Vec<T>,
    }

    impl<T> StronglyLIS<T>
    where
        T: Clone + Ord,
    {
        pub fn new() -> Self {
            Self { dp: vec![] }
        }

        pub fn push(&mut self, x: T) {
            let idx = self.dp.lower_bound(&x);
            if idx < self.dp.len() {
                self.dp[idx] = x;
            } else {
                self.dp.push(x);
            }
        }

        pub fn lis_len(&self) -> usize {
            self.dp.len()
        }
    }

    impl<T> Default for StronglyLIS<T>
    where
        T: Clone + Ord,
    {
        fn default() -> Self {
            StronglyLIS::new()
        }
    }

    impl<T> From<Vec<T>> for StronglyLIS<T>
    where
        T: Clone + Ord,
    {
        fn from(seq: Vec<T>) -> Self {
            let mut lis = Self::default();
            for x in seq {
                lis.push(x);
            }
            lis
        }
    }
}
