use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![];
    for a in 1..=162.min(n - 1) {
        let x = n - a;

        let mut fx = 0;
        let mut t = x;
        while t != 0 {
            fx += t % 10;
            t /= 10;
        }

        if fx == a {
            ans.push(x);
        }
    }

    ans.sort_unstable();

    println!("{}", ans.len());
    for elem in ans {
        println!("{}", elem);
    }
}
