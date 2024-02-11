use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
    }

    let mut sum = 0.0;
    for init_score in 1..=n {
        let req_num = (0..).find(|&i| init_score << i >= k).unwrap();
        sum += 2.0_f64.powi(-req_num);
    }

    let ans = sum / n as f64;
    println!("{}", ans);
}
