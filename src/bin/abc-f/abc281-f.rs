use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let ans = dfs(&aa, 30);
    println!("{}", ans);
}

fn dfs(aa: &[usize], digit: usize) -> usize {
    let mut zero = vec![];
    let mut one = vec![];
    for &a in aa {
        if a >> digit & 1 == 0 {
            zero.push(a);
        } else {
            one.push(a ^ 1 << digit);
        }
    }

    if digit == 0 {
        return ((!zero.is_empty() && !one.is_empty()) as usize) << digit;
    }

    match (zero.is_empty(), one.is_empty()) {
        (true, true) => 0,
        (true, false) => dfs(&one, digit - 1),
        (false, true) => dfs(&zero, digit - 1),
        (false, false) => (1 << digit) + dfs(&zero, digit - 1).min(dfs(&one, digit - 1)),
    }
}
