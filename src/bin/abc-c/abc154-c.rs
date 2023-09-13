use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.sort_unstable();
    aa.dedup();

    println!("{}", if aa.len() == n { "YES" } else { "NO" });
}
