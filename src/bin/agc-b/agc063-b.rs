use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut ans = 0;
    let mut stack = vec![];
    for left in (0..n).rev() {
        stack.push(left);

        for cnt in 0..n {
            if !stack.is_empty() && aa[*stack.last().unwrap()] == cnt {
                stack.pop();
            } else {
                break;
            }
        }

        ans += stack.last().unwrap_or(&n) - left;
    }

    println!("{}", ans);
}
