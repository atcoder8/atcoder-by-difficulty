use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        mm: [usize; n],
    }

    let sum: usize = mm.iter().sum();
    let min = *mm.iter().min().unwrap();
    let ans = n + (x - sum) / min;
    println!("{}", ans);
}
