use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut acc = 0;
    let mut acc_max = 0;
    let mut pos = 0;
    let mut ans = 0;

    for i in 0..n {
        acc += aa[i];
        acc_max = acc_max.max(acc);
        ans = ans.max(pos + acc_max);
        pos += acc;
    }

    println!("{}", ans);
}
