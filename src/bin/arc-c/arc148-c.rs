use im_rc::HashSet;
use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        pp: [Usize1; n - 1],
        queries: [[Usize1]; q],
    }

    let mut children = vec![vec![]; n];
    for (i, &p) in enumerate(&pp) {
        children[p].push(i + 1);
    }

    for up_coins in queries {
        let up_coin_set: HashSet<usize> = up_coins.into_iter().collect();

        let mut ans = 0;

        for &coin in &up_coin_set {
            ans += children[coin].len() + 1;
        }

        for &coin in &up_coin_set {
            if coin != 0 && up_coin_set.contains(&pp[coin - 1]) {
                ans -= 2;
            }
        }

        println!("{}", ans);
    }
}
