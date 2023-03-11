use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        cc: Chars,
    }

    let mut ans = 0;

    let mut left = 0;
    let mut right = n - 1;

    while left < right {
        while left < right && cc[left] != 'W' {
            left += 1;
        }

        while left < right && cc[right] != 'R' {
            right -= 1;
        }

        if left >= right {
            break;
        }

        ans += 1;

        left += 1;
        right -= 1;
    }

    println!("{}", ans);
}
