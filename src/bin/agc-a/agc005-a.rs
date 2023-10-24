use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    }

    let mut s_cnt = 0;
    let mut t_cnt = 0;
    for &c in &x {
        if c == 'S' {
            s_cnt += 1;
        } else if s_cnt >= 1 {
            s_cnt -= 1;
        } else {
            t_cnt += 1;
        }
    }

    println!("{}", s_cnt + t_cnt);
}
