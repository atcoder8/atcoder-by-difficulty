use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let divisors = find_divisors(m);
    let ans = divisors.into_iter().rev().find(|d| m / d >= n).unwrap();
    println!("{}", ans);
}

/// Creates a sequence consisting of the divisors of `n`.
pub fn find_divisors(n: usize) -> Vec<usize> {
    assert_ne!(n, 0, "`n` must be at least 1.");

    let mut divisors = vec![];

    for i in (1..).take_while(|&i| i <= n / i) {
        if n % i == 0 {
            divisors.push(i);

            if n / i != i {
                divisors.push(n / i);
            }
        }
    }

    divisors.sort_unstable();

    divisors
}
