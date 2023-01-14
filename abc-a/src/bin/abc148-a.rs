use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let ans = (1..=3).find(|&x| x != a && x != b).unwrap();
    println!("{}", ans);
}
