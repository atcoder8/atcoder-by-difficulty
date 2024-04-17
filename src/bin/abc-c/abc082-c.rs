use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut ans = 0;
    let mut counts = vec![0; n + 1];

    for &a in &aa {
        if a <= n {
            counts[a] += 1;
        } else {
            ans += 1;
        }
    }

    for i in 1..=n {
        ans += if counts[i] >= i {
            counts[i] - i
        } else {
            counts[i]
        };
    }

    println!("{}", ans);
}
