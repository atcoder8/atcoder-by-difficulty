use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0;
    println!("{}", if ans { "Possible" } else { "Impossible" });
}
