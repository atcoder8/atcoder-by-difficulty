use proconio::input;

fn main() {
    input! {
        (a, op, b): (i64, char, i64),
    }

    println!("{}", if op == '+' { a + b } else { a - b });
}
