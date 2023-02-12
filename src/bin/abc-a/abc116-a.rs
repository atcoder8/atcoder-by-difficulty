use proconio::input;

fn main() {
    input! {
        (ab, bc, _): (usize, usize, usize),
    }

    println!("{}", ab * bc / 2);
}
