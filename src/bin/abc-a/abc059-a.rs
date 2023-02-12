use proconio::{input, marker::Chars};

fn main() {
    input! {
        ss: [Chars; 3],
    }

    let ans = ss.iter().map(|s| s[0]).collect::<String>().to_uppercase();
    println!("{}", ans);
}
