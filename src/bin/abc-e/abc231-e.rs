use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        aa: [usize; n],
    }

    let mut lt = None;
    let mut ge = 0;
    for i in 0..n - 1 {
        let slope = aa[i + 1] / aa[i];
        let d = x % aa[i + 1] / aa[i];

        let mut next_lt = None;
        let mut next_ge = None;

        if let Some(lt) = lt {
            // lt -> lt
            update_cost(&mut next_lt, lt + slope - 1 - d);

            // lt -> ge
            if d < slope - 1 {
                update_cost(&mut next_ge, lt + d + 1);
            }
        }

        // ge -> lt
        if d > 0 {
            update_cost(&mut next_lt, ge + slope - d);
        }

        // ge -> ge
        update_cost(&mut next_ge, ge + d);

        lt = next_lt;
        ge = next_ge.unwrap();
    }

    let ans = match lt {
        Some(lt) => (lt + 1).min(ge),
        None => ge,
    } + x / aa[n - 1];
    println!("{}", ans);
}

fn update_cost(cost: &mut Option<usize>, cand_cost: usize) {
    if cost.is_none() || cand_cost < cost.unwrap() {
        *cost = Some(cand_cost);
    }
}
