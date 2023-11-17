use im_rc::HashMap;
use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, ma, mb): (usize, i64, i64),
        abc: [(i64, i64, usize); n],
    }

    let sum_abc = |abc: &[(i64, i64, usize)], bit: usize| {
        enumerate(abc)
            .filter(|&(i, _)| bit >> i & 1 == 1)
            .fold((0, 0, 0), |(sum_a, sum_b, sum_c), (_, (a, b, c))| {
                (sum_a + a, sum_b + b, sum_c + c)
            })
    };

    let abc1 = &abc[..n / 2];
    let abc2 = &abc[n / 2..];

    let mut map = HashMap::new();
    for bit in 1..1 << abc1.len() {
        let (sum_a, sum_b, sum_c) = sum_abc(&abc1, bit);

        let key = mb * sum_a - ma * sum_b;
        let min_cost = map.entry(key).or_insert(sum_c);
        *min_cost = (*min_cost).min(sum_c);
    }

    let mut ans = None;
    for bit in 0..1 << abc2.len() {
        let (sum_a, sum_b, sum_c) = sum_abc(&abc2, bit);

        if let Some(&cost1) = map.get(&(ma * sum_b - mb * sum_a)) {
            let sum_cost = cost1 + sum_c;
            if sum_cost != 0 {
                update_cost(&mut ans, sum_cost);
            }
        }

        if bit != 0 && mb * sum_a - ma * sum_b == 0 {
            update_cost(&mut ans, sum_c);
        }
    }

    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn update_cost<T>(cost: &mut Option<T>, cand_cost: T)
where
    T: Ord,
{
    if cost.is_none() || &cand_cost < cost.as_ref().unwrap() {
        *cost = Some(cand_cost);
    }
}
