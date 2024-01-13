use proconio::{input, marker::Usize1};

const MAX: usize = 2 * 10_usize.pow(5);

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut counts = [0; MAX];
    for &a in &aa {
        counts[a] += 1;
    }

    let mut ans = n * (n - 1) * (n - 2) / 6;
    for &cnt in &counts {
        if cnt >= 2 {
            ans -= cnt * (cnt - 1) / 2 * (n - cnt);

            if cnt >= 3 {
                ans -= cnt * (cnt - 1) * (cnt - 2) / 6;
            }
        }
    }

    println!("{}", ans);
}
