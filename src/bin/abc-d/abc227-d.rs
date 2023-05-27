use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let total_a: usize = aa.iter().sum();

    let mut ok = 0_usize;
    let mut ng = (total_a + k - 1) / k + 1;

    let is_ok = |x: usize| {
        let sum: usize = aa.iter().map(|&a| a.min(x)).sum();

        sum >= k * x
    };

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
