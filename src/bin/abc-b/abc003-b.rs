use itertools::izip;
use proconio::input;

const ATCODER: &str = "atcoder";

fn main() {
    input! {
        s: String,
        t: String,
    }

    let ans = izip!(s.chars(), t.chars()).all(|(c1, c2)| match (c1 == '@', c2 == '@') {
        (true, true) => true,
        (true, false) => ATCODER.contains(c2),
        (false, true) => ATCODER.contains(c1),
        (false, false) => c1 == c2,
    });

    println!("{}", if ans { "You can win" } else { "You will lose" });
}
