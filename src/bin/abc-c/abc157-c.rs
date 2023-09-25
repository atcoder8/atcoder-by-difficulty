use proconio::input;

fn main() {
    input! {
        (n, m): (u32, usize),
        sc: [(u32, u32); m]
    }

    let is_ok = |number: u32| {
        sc.iter()
            .all(|&(s, c)| number / 10_u32.pow(n - s) % 10 == c)
    };

    let mut range = if n == 1 {
        0..10
    } else if n == 2 {
        10..100
    } else {
        100..1000
    };

    let ans = range.find(|&number| is_ok(number));
    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}
