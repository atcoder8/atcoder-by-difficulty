use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    println!("{}", (0..t).map(|_| solve()).join("\n"));
}

fn solve() -> usize {
    input! {
        n: usize,
    }

    let is_ok = |n: usize, lim_k: usize| {
        let mut stack = vec![(n, lim_k)];
        while let Some((n, lim_k)) = stack.pop() {
            if n == 0 {
                return true;
            }

            for k in 1..=lim_k {
                for r in k..=(3 * k).min(n) {
                    if (n - r) % 10 == 0 {
                        stack.push(((n - r) / 10, k));
                    }
                }
            }
        }

        false
    };

    (1..).find(|&lim_k| is_ok(n, lim_k)).unwrap()
}
