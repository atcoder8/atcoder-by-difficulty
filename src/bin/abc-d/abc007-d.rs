use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = count_prohibited_numbers(b + 1) - count_prohibited_numbers(a);
    println!("{}", ans);
}

fn count_prohibited_numbers(x: usize) -> usize {
    // less[bit]: bitで表される各桁のみが4または9である数の個数
    let mut less = vec![0; 1 << 19];
    let mut equal = 0;

    for i in (0..=18).rev() {
        let d = x / 10_usize.pow(i) % 10;

        let mut next_less = vec![0; 1 << 19];

        // less -> less
        for bit in 0..(1 << 18) {
            next_less[bit] += 8 * less[bit];
            next_less[bit | 1 << i] += 2 * less[bit];
        }

        // equal -> less
        for j in 0..d {
            next_less[equal | ((j == 4 || j == 9) as usize) << i] += 1;
        }

        // equal -> equal
        equal |= ((d == 4 || d == 9) as usize) << i;

        less = next_less;
    }

    less[1..].iter().sum()
}
