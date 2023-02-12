use proconio::input;

fn main() {
    input! {
        mut seq: [i64; 3],
    }

    seq.sort_unstable();

    println!("{}", if seq[0] == seq[1] { seq[2] } else { seq[0] });
}
