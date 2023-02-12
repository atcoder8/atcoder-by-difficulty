use proconio::input;

fn main() {
    input! {
        mut seq: [usize; 3],
    }

    seq.sort_unstable();

    let ans = seq[0] == 5 && seq[1] == 5 && seq[2] == 7;
    println!("{}", if ans { "YES" } else { "NO" });
}
