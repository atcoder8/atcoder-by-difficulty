use itertools::iproduct;
use proconio::input;

const RADIX: usize = 10;
const MAX_DIGIT_NUM: usize = 6;
const TENS: [usize; 7] = [1, 10, 100, 1000, 10000, 100000, 1000000];

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut prefix_sum = vec![0; TENS[MAX_DIGIT_NUM]];
    for &a in &aa {
        prefix_sum[a] += 1;
    }
    for d in 0..MAX_DIGIT_NUM {
        for (lower_digit, upper_digit) in iproduct!(0..TENS[d], 0..TENS[MAX_DIGIT_NUM - 1 - d]) {
            let base = lower_digit + upper_digit * TENS[d + 1];
            for i in 0..RADIX - 1 {
                prefix_sum[base + (i + 1) * TENS[d]] += prefix_sum[base + i * TENS[d]];
            }
        }
    }

    let ans = aa
        .iter()
        .map(|&a| {
            prefix_sum[TENS[MAX_DIGIT_NUM] - 1 - a]
                - (0..MAX_DIGIT_NUM).all(|d| a / TENS[d] % RADIX * 2 < RADIX) as usize
        })
        .sum::<usize>()
        / 2;
    println!("{}", ans);
}
