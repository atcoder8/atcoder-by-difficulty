use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let calc_cost = |init_positive: bool| {
        let mut cost = 0;
        let mut sum = 0;
        let mut positive = init_positive;

        for &a in &aa {
            sum += a;

            if positive && sum <= 0 {
                cost += -sum + 1;
                sum = 1;
            } else if !positive && sum >= 0 {
                cost += sum + 1;
                sum = -1;
            }

            positive = !positive;
        }

        cost
    };

    let ans = calc_cost(true).min(calc_cost(false));
    println!("{}", ans);
}
