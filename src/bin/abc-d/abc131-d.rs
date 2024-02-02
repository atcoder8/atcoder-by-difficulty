use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_unstable_by_key(|x| x.1);

    let mut cur = 0;
    for &(a, b) in &ab {
        if cur + a > b {
            return false;
        }

        cur += a;
    }

    true
}
