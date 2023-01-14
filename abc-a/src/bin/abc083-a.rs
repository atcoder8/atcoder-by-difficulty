use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    let ans = match (a + b).cmp(&(c + d)) {
        std::cmp::Ordering::Less => "Right",
        std::cmp::Ordering::Equal => "Balanced",
        std::cmp::Ordering::Greater => "Left",
    };
    println!("{}", ans);
}
