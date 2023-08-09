use proconio::input;

fn main() {
    input! {
        w: String,
    }

    let ans: String = w.chars().filter(|&c| !"aiueo".contains(c)).collect();
    println!("{}", ans);
}
