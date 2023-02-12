use proconio::input;

fn main() {
    input! {
        mut seq: [usize; 3],
    }

    seq.sort_unstable();
    seq.dedup();

    println!("{}", seq.len());
}
