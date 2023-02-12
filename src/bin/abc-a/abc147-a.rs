use proconio::input;

fn main() {
    input! {
        (a1, a2, a3): (usize, usize, usize),
    }

    println!("{}", if a1 + a2 + a3 >= 22 { "bust" } else { "win" });
}
