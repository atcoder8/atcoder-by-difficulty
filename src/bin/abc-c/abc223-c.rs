use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n],
    }

    let mut ans = 0.0;
    let mut rem = ab.iter().map(|&(a, b)| a / b).sum::<f64>() / 2.0;
    for &(a, b) in &ab {
        let t = a / b;

        if t > rem {
            ans += b * rem;
            break;
        }

        ans += a;
        rem -= t;
    }

    println!("{}", ans);
}
