use proconio::input;

fn main() {
    input! {
        (mut a, mut b): (usize, usize),
    }

    if a == b {
        println!("1");
        std::process::exit(0);
    }

    if a < b {
        std::mem::swap(&mut a, &mut b);
    }

    let mut ans = 0;
    while b > 0 {
        let divisors = find_divisors(a - b);
        let min_t = divisors
            .iter()
            .filter(|&&d| d >= 2)
            .map(|&d| (a % d, d))
            .min_by_key(|x| x.0);
        if let Some((min_t, d)) = min_t {
            ans += min_t;
            a = (a - min_t) / d;
            b = (b - min_t) / d;
        } else {
            ans += b;
            break;
        }
    }

    println!("{}", ans);
}

/// Creates a sequence consisting of the divisors of `n`.
pub fn find_divisors(n: usize) -> Vec<usize> {
    assert_ne!(n, 0, "`n` must be at least 1.");

    let mut divisors = vec![];

    for i in (1..).take_while(|i| i * i <= n) {
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
