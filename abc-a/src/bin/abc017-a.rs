use proconio::input;

fn main() {
    input! {
        se: [(usize, usize); 3],
    }

    let ans: usize = se.iter().map(|&(s, e)| s * e / 10).sum();
    println!("{}", ans);
}
