use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            (n, a, b, c, d): (usize, usize, usize, usize, usize),
        }

        let ans = rec(n, [(a, 2), (b, 3), (c, 5)], d, &mut HashMap::new());
        println!("{}", ans);
    }
}

fn rec(
    n: usize,
    change_costs: [(usize, usize); 3],
    d: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return d;
    }

    if let Some(&sum_cost) = memo.get(&n) {
        return sum_cost;
    }

    let mut min_sum_cost = n.saturating_mul(d);

    for (cost, div) in change_costs {
        if n < div {
            continue;
        }

        let floor_mul = n / div * div;
        let next_n = floor_mul / div;

        if next_n >= n {
            continue;
        }

        let sub_cost = (n - floor_mul) * d;
        let diff = floor_mul - next_n;

        let sum_cost = sub_cost
            + if d <= cost / diff { diff * d } else { cost }
            + rec(next_n, change_costs, d, memo);

        min_sum_cost = min_sum_cost.min(sum_cost);
    }

    for (cost, div) in change_costs {
        let ceil_mul = (n + div - 1) / div * div;
        let next_n = ceil_mul / div;

        if next_n >= n {
            continue;
        }

        let add_cost = (ceil_mul - n) * d;
        let diff = ceil_mul - next_n;

        let sum_cost = add_cost
            + if d <= cost / (ceil_mul - next_n) {
                diff * d
            } else {
                cost
            }
            + rec(next_n, change_costs, d, memo);

        min_sum_cost = min_sum_cost.min(sum_cost);
    }

    memo.insert(n, min_sum_cost);

    min_sum_cost
}
