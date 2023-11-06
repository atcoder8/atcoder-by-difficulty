use itertools::{enumerate, izip, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (_n, k, c): (usize, usize, usize),
        s: Chars,
    }

    let mut work = 0;
    let mut rest = c;
    let mut leftmost = vec![0; k];
    for (i, &ch) in enumerate(&s) {
        if ch == 'o' && rest >= c {
            leftmost[work] = i;
            work += 1;
            rest = 0;

            if work == k {
                break;
            }
        } else {
            rest += 1;
        }
    }

    let mut work = k;
    let mut rest = c;
    let mut rightmost = vec![0; k + 1];
    for (i, &ch) in enumerate(&s).rev() {
        if ch == 'o' && rest >= c {
            work -= 1;
            rest = 0;
            rightmost[work] = i;

            if work == 0 {
                break;
            }
        } else {
            rest += 1;
        }
    }

    let mut ans =
        izip!(leftmost, rightmost)
            .filter_map(|(day1, day2)| if day1 == day2 { Some(day1 + 1) } else { None });
    println!("{}", ans.join("\n"));
}
