use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let calc_min_cost = |init: [Option<usize>; 2]| {
        let mut dp = init;
        for &a in &aa[1..] {
            dp = [
                dp[1],
                Some(dp.iter().filter_map(|&cost| cost).min().unwrap() + a),
            ];
        }

        dp
    };

    let cost1 = calc_min_cost([Some(0), None])[1].unwrap();
    let cost_2 = calc_min_cost([None, Some(aa[0])])
        .iter()
        .filter_map(|&cost| cost)
        .min()
        .unwrap();

    println!("{}", cost1.min(cost_2));
}
