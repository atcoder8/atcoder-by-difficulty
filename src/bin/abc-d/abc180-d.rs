use proconio::input;

fn main() {
    input! {
        (x, y, a, b): (usize, usize, usize, usize),
    }

    let mut ans = 0;

    let mut strong = x;
    while strong < y / a && strong < (strong + b) / a {
        strong *= a;
        ans += 1;
    }

    ans += (y - strong - 1) / b;

    println!("{}", ans);
}
