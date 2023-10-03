use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut ans = String::new();
    let mut pending_b = false;
    for &c in &s {
        match c {
            'A' => ans.push('A'),
            'B' => {
                if pending_b {
                    ans.push('A');
                }
                pending_b = !pending_b;
            }
            'C' => {
                if pending_b {
                    ans.push('B');
                    pending_b = false;
                }
                ans.push('C');
            }
            _ => panic!(),
        }
    }
    if pending_b {
        ans.push('B');
    }

    println!("{}", ans);
}
