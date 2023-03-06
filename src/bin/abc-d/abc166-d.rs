use proconio::input;

fn main() {
    let (a, b) = solve();
    println!("{} {}", a, b);
}

fn solve() -> (i64, i64) {
    input! {
        x: i64,
    }

    for a in -118_i64..=119 {
        for b in -119_i64..=118 {
            if a.pow(5) - b.pow(5) == x {
                return (a, b);
            }
        }
    }

    unreachable!()
}
