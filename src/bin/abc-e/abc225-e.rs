use itertools::Itertools;
use proconio::input;

use crate::amplitude::Amplitude;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    let sections = xy
        .iter()
        .map(|&(x, y)| (Amplitude::new(x, y - 1), Amplitude::new(x - 1, y)))
        .sorted_by_key(|x| x.1);

    let mut ans = 0;
    let mut cur = Amplitude::new(1, 0);
    for (left, right) in sections {
        if left >= cur {
            ans += 1;
            cur = right;
        }
    }

    println!("{}", ans);
}

pub mod amplitude {
    fn gcd(a: usize, b: usize) -> usize {
        let mut a = a;
        let mut b = b;

        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }

        a
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Amplitude {
        pub x: usize,
        pub y: usize,
    }

    impl PartialOrd for Amplitude {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            (other.x * self.y).partial_cmp(&(self.x * other.y))
        }
    }

    impl Ord for Amplitude {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    impl Amplitude {
        pub fn new(x: usize, y: usize) -> Self {
            assert_ne!((x, y), (0, 0));

            let g = gcd(x, y);

            Self { x: x / g, y: y / g }
        }
    }
}
