use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let mut less = vec![vec![0; 17]; 2];
    let mut equal = vec![vec![0; 17]; 2];
    if n[0] == '1' {
        equal[1][1] = 1;
    } else {
        less[1][1] = 1;
    }
    for (i, &c) in n.iter().enumerate().skip(1) {
        let mut next_less = vec![vec![0; 17]; 2];
        let mut next_equal = vec![vec![0; 17]; 2];

        let d = c.to_digit(10).unwrap() as usize;

        if i >= 1 || d >= 2 {
            next_less[1][1] += 1;
        }

        // less -> less
        for j in 1..=15 {
            next_less[0][j] += 10 * less[0][j] + 9 * less[1][j];
            next_less[1][j + 1] += less[1][j];
        }

        // equal -> less
        for j in 0..d {
            for k in 1..=15 {
                next_less[0][k] += equal[0][k];

                if j == 1 {
                    next_less[1][k + 1] += equal[1][k];
                } else {
                    next_less[0][k] += equal[1][k];
                }
            }
        }

        // equal -> equal
        for j in 1..=15 {
            next_equal[0][j] += equal[0][j];
            if d == 1 {
                next_equal[1][j + 1] += equal[1][j];
            } else {
                next_equal[0][j] += equal[1][j];
            }
        }

        less = next_less;
        equal = next_equal;
    }

    let mut ans = 0;
    for i in 1..=15 {
        ans += i * (less[0][i] + less[1][i] + equal[0][i] + equal[1][i]);
    }
    println!("{}", ans);
}
