use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        aa: [usize; n],
    }

    if aa[0] != 0 || aa.windows(2).any(|window| window[1] >= window[0] + 2) {
        return None;
    }

    let mut ans = 0;
    for window in aa.windows(2) {
        if window[1] == window[0] + 1 {
            ans += 1;
        } else {
            ans += window[1];
        }
    }

    Some(ans)
}
