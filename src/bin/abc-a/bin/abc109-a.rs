use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize,),
    }

    let ans = a * b % 2 == 1;
    println!("{}", if ans { "Yes" } else { "No" });
}
