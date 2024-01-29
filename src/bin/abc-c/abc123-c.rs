use proconio::input;

fn main() {
    input! {
        n: usize,
        flow_limits: [usize; 5],
    }

    let min_flow = flow_limits.into_iter().min().unwrap().min(n);
    println!("{}", (n + min_flow - 1) / min_flow + 4);
}
