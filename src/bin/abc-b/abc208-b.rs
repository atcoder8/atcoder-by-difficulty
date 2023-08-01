use proconio::input;

fn main() {
    input! {
        p: usize,
    }

    let mut frac = vec![1];
    for i in 1..=10 {
        frac.push(frac[i - 1] * i);
    }

    let mut ans = 0;
    let mut rem = p;
    for i in (1..=10).rev() {
        let q = rem / frac[i];
        ans += q;
        rem -= frac[i] * q;
    }

    println!("{}", ans);
}
