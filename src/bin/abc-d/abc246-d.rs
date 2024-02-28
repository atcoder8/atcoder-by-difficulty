use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 10_usize.pow(18);
    for a in 0..=10_usize.pow(6) {
        if a.pow(3) >= n {
            ans = ans.min(a.pow(3));
            continue;
        }

        let mut ok = 10_usize.pow(6);
        let mut ng = 0_usize;
        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) / 2;

            if f(a, mid) >= n {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ans = ans.min(f(a, ok));
    }

    println!("{}", ans);
}

fn f(a: usize, b: usize) -> usize {
    a.pow(3) + a.pow(2) * b + a * b.pow(2) + b.pow(3)
}
