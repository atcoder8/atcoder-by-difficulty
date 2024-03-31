use proconio::input;

fn main() {
    input! {
        aa: [usize; 10],
    }

    let ans = (0..3).fold(0, |prev, _| aa[prev]);
    println!("{}", ans);
}
