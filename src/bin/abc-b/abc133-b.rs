use itertools::{izip, Itertools};
use proconio::input;

fn main() {
    input! {
        (n, d): (usize, usize),
        xx: [[i64; d]; n],
    }

    let calc_sq_dist = |i: usize, j: usize| -> u64 {
        izip!(&xx[i], &xx[j])
            .map(|(&x1, &x2)| x1.abs_diff(x2).pow(2))
            .sum()
    };

    let ans = (0..n)
        .tuple_combinations()
        .filter(|&(i, j)| calc_sq_dist(i, j).is_square())
        .count();
    println!("{}", ans);
}

pub trait IsSquare {
    fn is_square(&self) -> bool;
}

macro_rules! impl_is_square_for_unsigned_integer {
    ( $( $uint: ty ), * ) => {
        $(
            impl IsSquare for $uint {
                fn is_square(&self) -> bool {
                    if *self <= 1 {
                        return true;
                    }

                    let mut left = 1;
                    let mut right = *self;

                    while left < right {
                        let mid = left + (right - left) / 2;

                        match mid.cmp(&(self / mid)) {
                            std::cmp::Ordering::Less => left = mid + 1,
                            std::cmp::Ordering::Equal => return self % mid == 0,
                            std::cmp::Ordering::Greater => right = mid,
                        }
                    }

                    left == self / left && self % left == 0
                }
            }
        )*
    };
}

impl_is_square_for_unsigned_integer!(u8, u16, u32, u64, u128, usize);
