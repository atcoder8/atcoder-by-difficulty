use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let even_num = k / 2;

    println!("{}", even_num * (k - even_num));
}
