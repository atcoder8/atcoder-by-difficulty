use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut sum = aa[0];
    let mut rem: i64 = aa[1..].iter().sum();

    let mut ans = None;
    for &a in aa[1..].iter() {
        let cost = sum.abs_diff(rem);
        if ans.is_none() || cost < ans.unwrap() {
            ans = Some(cost);
        }

        sum += a;
        rem -= a;
    }

    println!("{}", ans.unwrap());
}
