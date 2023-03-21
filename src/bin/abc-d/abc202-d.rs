use proconio::input;

fn main() {
    input! {
        (a, b, k): (usize, usize, usize),
    }

    let s = a + b;

    let mut memo = vec![vec![None; b + 1]; s];
    let mut chars = vec!['a'; s];
    let mut rem_b = b;
    let mut rem_k = k;

    for i in 0..s {
        let comb = combinations(&mut memo, s - i - 1, rem_b);

        if comb < rem_k {
            chars[i] = 'b';
            rem_b -= 1;
            rem_k -= comb;
        }
    }

    let ans: String = chars.iter().collect();
    println!("{}", ans);
}

fn combinations(memo: &mut Vec<Vec<Option<usize>>>, n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }

    if k == 0 || n == k {
        return 1;
    }

    if let Some(ret) = memo[n][k] {
        return ret;
    }

    let ret: usize = ((k - 1)..n).map(|i| combinations(memo, i, k - 1)).sum();
    memo[n][k] = Some(ret);

    ret
}
