use proconio::input;

fn main() {
    input! {
        (mut a, mut b): (usize, usize),
    }

    if a == 1 {
        a = 14;
    }

    if b == 1 {
        b = 14;
    }

    let ans = match a.cmp(&b) {
        std::cmp::Ordering::Less => "Bob",
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Alice",
    };
    println!("{}", ans);
}
