use proconio::input;

fn main() {
    input! {
        (n, l, r): (usize, usize, usize),
        aa: [usize; n],
    }

    let grundy = |a: usize| a % (l + r) / l;

    let ans = aa.iter().fold(0, |acc, &a| acc ^ grundy(a)) != 0;
    println!("{}", if ans { "First" } else { "Second" });
}
