use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let ans = (x..).find(|&i| primality_test(i)).unwrap();
    println!("{}", ans);
}

/// Returns `true` if and only if `n` is prime.
pub fn primality_test(n: usize) -> bool {
    n >= 2 && (2_usize..).take_while(|&i| i <= n / i).all(|i| n % i != 0)
}
