use proconio::input;

fn main() {
    input! {
        mut seq: [usize; 3],
    }

    seq.sort_unstable();

    println!("{}", seq[0] + seq[1]);
}
