use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut pp: [usize; 2 * n],
    }

    let mut ans = vec![];
    for i in 0..n {
        let mid = 2 * i + 1;
        let max_pos = 2 * i
            + pp[(2 * i)..(2 * i + 3).min(2 * n)]
                .iter()
                .position_max()
                .unwrap();
        if max_pos != mid {
            let x = max_pos.min(mid);
            pp.swap(x, x + 1);
            ans.push(x + 1);
        }
    }

    println!("{}\n{}", ans.len(), ans.iter().join(" "));
}
