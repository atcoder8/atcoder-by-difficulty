use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            nabxyz: [usize; 6],
        }

        let (n, mut a, mut b, x, mut y, mut z) = nabxyz.into_iter().collect_tuple().unwrap();

        y = y.min(a * x);
        z = z.min(b * x);
        if y * b > z * a {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut y, &mut z);
        }

        let calc_cost = |add_a_num: usize, add_b_num: usize| {
            x * (n - add_a_num * a - add_b_num * b) + y * add_a_num + z * add_b_num
        };

        let ans = if n / a <= a - 1 {
            (0..=n / a)
                .map(|add_a_num| calc_cost(add_a_num, (n - a * add_a_num) / b))
                .min()
                .unwrap()
        } else {
            (0..a)
                .map(|add_b_num| calc_cost((n - b * add_b_num) / a, add_b_num))
                .min()
                .unwrap()
        };
        println!("{}", ans);
    }
}
