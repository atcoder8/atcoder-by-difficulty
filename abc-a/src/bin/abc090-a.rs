use proconio::{input, marker::Chars};

fn main() {
    input! {
        ccc: [Chars; 3],
    }

    println!("{}{}{}", ccc[0][0], ccc[1][1], ccc[2][2]);
}
