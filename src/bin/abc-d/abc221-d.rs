use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut events = vec![];
    for &(a, b) in &ab {
        events.push((a, true));
        events.push((a + b, false));
    }
    events.sort_unstable_by_key(|x| x.0);

    let mut ans = vec![0_usize; n];
    let mut cnt = 0;
    let mut prev = 0;

    for &(date, login) in &events {
        if cnt != 0 {
            ans[cnt - 1] += date - prev;
        }

        if login {
            cnt += 1;
        } else {
            cnt -= 1;
        }

        prev = date;
    }

    println!("{}", ans.iter().map(|x| x.to_string()).join(" "));
}
