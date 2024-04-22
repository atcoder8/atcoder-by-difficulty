use proconio::input;

fn main() {
    input! {
        s: String,
        k: usize,
    }

    let ans = s.chars().take(k).find(|&c| c != '1').unwrap_or('1');
    println!("{}", ans);
}
