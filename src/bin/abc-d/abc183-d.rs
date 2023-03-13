use proconio::input;

fn main() {
    input! {
        (n, w): (usize, usize),
        stp: [(usize, usize, i64); n],
    }

    let max_t = stp.iter().map(|&(_, t, _)| t).max().unwrap();

    let mut imos = vec![0; max_t + 1];
    for &(s, t, p) in &stp {
        imos[s] += p;
        imos[t] -= p;
    }

    for i in 1..=max_t {
        imos[i] += imos[i - 1];
    }

    let ans = imos.iter().all(|&x| x as usize <= w);
    println!("{}", if ans { "Yes" } else { "No" });
}
