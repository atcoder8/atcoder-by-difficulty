use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![];
    let mut t = n;
    while t != 0 {
        if t % 2 == 1 {
            ans.push('A');
        }
        t -= t % 2;
        if t == 0 {
            break;
        }
        ans.push('B');
        t /= 2;
    }
    let ans: String = ans.iter().rev().collect();
    println!("{}", ans);
}
