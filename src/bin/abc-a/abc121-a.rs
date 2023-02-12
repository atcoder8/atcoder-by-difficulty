use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        (rows, cols): (usize, usize),
    }

    println!("{}", h * w - (rows * w + cols * h - rows * cols));
}
