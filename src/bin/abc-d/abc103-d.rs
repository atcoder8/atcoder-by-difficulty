use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m],
    }

    let mut left = [0; 10_usize.pow(5) + 1];
    for &(a, b) in &ab {
        left[b] = left[b].max(a);
    }

    let mut ans = 0;
    let mut lower = 1;
    for idx in 1..=n {
        if left[idx] >= lower {
            ans += 1;
            lower = idx;
        }
    }

    println!("{}", ans);
}
