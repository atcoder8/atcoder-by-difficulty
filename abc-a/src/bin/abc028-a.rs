use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = if n <= 59 {
        "Bad"
    } else if n <= 89 {
        "Good"
    } else if n <= 99 {
        "Great"
    } else {
        "Perfect"
    };

    println!("{}", ans);
}
