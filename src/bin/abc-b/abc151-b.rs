use proconio::input;

fn main() {
    input! {
        (n, k, m): (usize, usize, usize),
        aa: [usize; n - 1],
    }

    let sum_a: usize = aa.iter().sum::<usize>();
    let req = (n * m).saturating_sub(sum_a);

    if req <= k {
        println!("{}", req);
    } else {
        println!("-1");
    }
}
