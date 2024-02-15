use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let mut ans = 0;
    let mut sum = 0;
    let mut right = 0;
    for left in 0..n {
        while right < n && sum < k {
            sum += aa[right];
            right += 1;
        }

        if sum < k {
            break;
        }

        ans += n - right + 1;
        sum -= aa[left];
    }

    println!("{}", ans);
}
