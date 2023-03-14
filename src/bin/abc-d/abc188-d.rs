use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, c): (usize, usize),
        mut abc: [(Usize1, usize, usize); n],
    }

    let mut events = vec![];
    for &(a, b, c) in &abc {
        events.push((a, c, true));
        events.push((b, c, false));
    }
    events.sort_unstable_by_key(|x| x.0);

    let mut prev_day = 0;
    let mut acc_cost = 0;
    let mut ans = 0;
    for &(day, cost, is_join) in &events {
        ans += acc_cost.min(c) * (day - prev_day);

        if is_join {
            acc_cost += cost;
        } else {
            acc_cost -= cost;
        }

        prev_day = day;
    }

    println!("{}", ans);
}
