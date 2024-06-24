use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let solve = |init: char| {
        let mut cur = init;
        let mut cnt = 0_usize;
        for &c in &s {
            if c != cur {
                cnt += 1;
            }

            cur = if cur == '0' { '1' } else { '0' };
        }

        cnt
    };

    let ans = solve('0').min(solve('1'));
    println!("{}", ans);
}
