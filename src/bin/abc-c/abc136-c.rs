use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        mut hh: [usize; n],
    }

    for i in (0..n - 1).rev() {
        if hh[i] > hh[i + 1] {
            if hh[i] - hh[i + 1] >= 2 {
                return false;
            }

            hh[i] -= 1;
        }
    }

    true
}
