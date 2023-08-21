use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = match n {
        1..=125 => 4,
        126..=211 => 6,
        212..=214 => 8,
        _ => unreachable!(),
    };
    println!("{}", ans);
}
