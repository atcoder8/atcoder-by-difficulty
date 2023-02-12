use proconio::input;

fn main() {
    input! {
        (a, b): (char, char),
    }

    let ans = !((a == 'H') ^ (b == 'H'));
    println!("{}", if ans { "H" } else { "D" });
}
