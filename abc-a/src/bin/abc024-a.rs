use proconio::input;

fn main() {
    input! {
        (a, b, c, k): (usize, usize, usize, usize),
        (s, t): (usize, usize),
    }

    let ans = a * s + b * t - if s + t >= k { (s + t) * c } else { 0 };
    println!("{}", ans);
}
