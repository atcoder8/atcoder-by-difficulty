use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let ans = (0..=n).find(|&i| !aa.contains(&i)).unwrap();
    println!("{}", ans);
}
