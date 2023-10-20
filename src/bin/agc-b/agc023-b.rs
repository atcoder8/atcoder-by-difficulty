use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        sss: [Chars; n],
    }

    let is_match = |row_diff: usize| {
        for row in 0..n {
            for col in (row + 1)..n {
                if sss[(row + row_diff) % n][col] != sss[(col + row_diff) % n][row] {
                    return false;
                }
            }
        }

        true
    };

    let ans = (0..n).filter(|&row_diff| is_match(row_diff)).count() * n;
    println!("{}", ans);
}
