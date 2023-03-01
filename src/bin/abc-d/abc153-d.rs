use proconio::input;

fn main() {
    input! {
        h: usize,
    }

    let ans = (1_usize << (0_usize..).find(|&i| h >> i == 0).unwrap()) - 1;
    println!("{}", ans);
}
