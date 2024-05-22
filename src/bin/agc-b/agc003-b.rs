use proconio::input;
fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut ans = 0;
    let mut rem = 0;
    for &a in &aa {
        let diff_pair_num = rem.min(a);
        let same_pair_num = (a - diff_pair_num) / 2;
        ans += diff_pair_num + same_pair_num;
        rem = a - (diff_pair_num + 2 * same_pair_num);
    }

    println!("{}", ans);
}
