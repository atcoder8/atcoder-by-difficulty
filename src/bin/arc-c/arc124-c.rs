use itertools::iproduct;
use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let (a0, b0) = ab[0];
    let divisors1 = find_divisors(a0);
    let divisors2 = find_divisors(b0);

    let mut ans = 0;
    for (x, y) in iproduct!(divisors1, divisors2) {
        let is_ok = |a: usize, b: usize| a % x == 0 && b % y == 0 || a % y == 0 && b % x == 0;
        if ab.iter().all(|&(a, b)| is_ok(a, b)) {
            ans = ans.max(x.lcm(&y));
        }
    }
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
