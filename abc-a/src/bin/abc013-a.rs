use proconio::input;

fn main() {
    input! {
        x: char,
    }

    println!("{}", x as u8 - b'A' + 1);
}
