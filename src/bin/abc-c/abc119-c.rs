use proconio::input;

use crate::subset_bit_iter::SubsetBitIter;

fn main() {
    input! {
        (n, dest_lengths): (usize, [usize; 3]),
        ll: [usize; n],
    }

    let calc_cost = |bit: usize, dest_length: usize| {
        let sum_length: usize = (0..n).filter(|&i| bit >> i & 1 == 1).map(|i| ll[i]).sum();

        10 * (bit.count_ones() as usize - 1) + dest_length.abs_diff(sum_length)
    };

    let mut ans = None;
    let overall_bit = (1 << n) - 1;
    for bit1 in SubsetBitIter::new(overall_bit) {
        if bit1 == 0 {
            continue;
        }

        let cost1 = calc_cost(bit1, dest_lengths[0]);

        for bit2 in SubsetBitIter::new(overall_bit ^ bit1) {
            if bit2 == 0 {
                continue;
            }

            let cost2 = calc_cost(bit2, dest_lengths[1]);

            for bit3 in SubsetBitIter::new(overall_bit ^ bit1 ^ bit2) {
                if bit3 == 0 {
                    continue;
                }

                let cost3 = calc_cost(bit3, dest_lengths[2]);

                let sum_cost = cost1 + cost2 + cost3;
                if ans.is_none() || sum_cost < ans.unwrap() {
                    ans = Some(sum_cost);
                }
            }
        }
    }

    println!("{}", ans.unwrap());
}

pub mod subset_bit_iter {
    #[derive(Debug, Clone, Copy)]
    pub struct SubsetBitIter {
        first: bool,
        overall_bit: usize,
        subset_bit: usize,
    }

    impl Iterator for SubsetBitIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.first {
                self.first = false;

                return Some(self.overall_bit);
            }

            if self.subset_bit == 0 {
                return None;
            }

            self.subset_bit = (self.subset_bit - 1) & self.overall_bit;

            Some(self.subset_bit)
        }
    }

    impl SubsetBitIter {
        pub fn new(overall_bit: usize) -> Self {
            Self {
                first: true,
                overall_bit,
                subset_bit: overall_bit,
            }
        }
    }
}
