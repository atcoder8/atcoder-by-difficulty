use proconio::input;

fn main() {
    input! {
        (a, b, k): (usize, usize, usize),
    }

    let mut ans = 0;
    let mut cur = a;
    while cur < b {
        ans += 1;
        cur *= k;
    }

    println!("{}", ans);
}
