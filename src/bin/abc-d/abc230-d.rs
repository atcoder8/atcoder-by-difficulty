use proconio::input;

fn main() {
    input! {
        (n, d): (usize, usize),
        mut lr: [(usize, usize); n],
    }

    lr.sort_unstable_by_key(|x| x.1);

    let mut ans = 1;
    let mut right = lr[0].1;
    for &(l, r) in &lr {
        if l > right + d - 1 {
            ans += 1;
            right = r;
        }
    }

    println!("{}", ans);
}
