use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        (r, c): (usize, usize),
    }

    let ans = [r > 1, r < h, c > 1, c < w].iter().filter(|&&v| v).count();
    println!("{}", ans);
}
