use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    let ans = match (b * c).cmp(&(a * d)) {
        std::cmp::Ordering::Less => "AOKI",
        std::cmp::Ordering::Equal => "DRAW",
        std::cmp::Ordering::Greater => "TAKAHASHI",
    };

    println!("{}", ans);
}
