use proconio::input;

fn main() {
    input! {
        (x, a): (usize, usize),
    }

    println!("{}", if x < a { 0 } else { 10 });
}
