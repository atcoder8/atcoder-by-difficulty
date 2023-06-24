use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, x, m): (usize, usize, usize),
    }

    let init = (0..m)
        .map(|i| {
            let fi = i.pow(2) % m;

            (fi, fi)
        })
        .collect_vec();
    let mut doubling = vec![init];
    for i in 0..33 {
        let next = doubling[i]
            .iter()
            .map(|&(mid, acc1)| {
                let (to, acc2) = doubling[i][mid];

                (to, acc1 + acc2)
            })
            .collect_vec();
        doubling.push(next);
    }

    let mut ans = x;
    let mut cur = x;
    for i in 0..34 {
        if ((n - 1) >> i) & 1 == 1 {
            let (next, acc) = doubling[i][cur];
            cur = next;
            ans += acc;
        }
    }

    println!("{}", ans);
}
