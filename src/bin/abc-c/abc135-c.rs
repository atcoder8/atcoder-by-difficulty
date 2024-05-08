use itertools::izip;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n + 1],
        bb: [usize; n],
    }

    let mut ans = 0;
    let mut carry = 0;

    for (&a, &b) in izip!(&aa, &bb) {
        let score = a.min(carry + b);
        ans += score;
        carry = (carry + b - score).min(b);
    }
    ans += aa[n].min(carry);

    println!("{}", ans);
}
