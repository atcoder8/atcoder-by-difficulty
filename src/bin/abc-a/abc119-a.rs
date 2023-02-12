use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let split_s = s.split("/").collect_vec();
    let month: usize = split_s[1].parse().unwrap();

    let ans = if month <= 4 { "Heisei" } else { "TBD" };
    println!("{}", ans);
}
