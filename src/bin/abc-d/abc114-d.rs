use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let counts = (2..=n)
        .filter(|&i| primality_test(i))
        .map(|p| {
            let mut cnt = 0;
            let mut t = n;

            while t >= p {
                let divided = t / p;
                cnt += divided;
                t = divided;
            }

            cnt
        })
        .collect_vec();

    let comb_num_1 = counts
        .iter()
        .cloned()
        .permutations(3)
        .filter(|triple| triple[0] >= 2 && triple[1] >= 4 && triple[2] >= 4)
        .count()
        / 2;
    let comb_num_2 = counts
        .iter()
        .cloned()
        .permutations(2)
        .filter(|pair| pair[0] >= 2 && pair[1] >= 24)
        .count();
    let comb_num_3 = counts
        .iter()
        .cloned()
        .permutations(2)
        .filter(|pair| pair[0] >= 4 && pair[1] >= 14)
        .count();
    let comb_num_4 = counts.iter().filter(|&&cnt| cnt >= 74).count();

    let ans = comb_num_1 + comb_num_2 + comb_num_3 + comb_num_4;
    println!("{}", ans);
}

/// Returns `true` if and only if `n` is prime.
pub fn primality_test(n: usize) -> bool {
    n >= 2 && (2_usize..).take_while(|&i| i <= n / i).all(|i| n % i != 0)
}
