use proconio::input;

const MOD: usize = 2019;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (l, r): (usize, usize),
    }

    if r - l >= MOD {
        return 0;
    }

    let mut ans = MOD;
    for i in l..r {
        for j in i + 1..=r {
            ans = ans.min(i * j % MOD);
        }
    }

    ans
}
