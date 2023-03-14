use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let mut t = 1_usize;
    let mut f = 1_usize;

    for s in &ss {
        let (next_t, next_f) = if s == "AND" {
            (t, t + 2 * f)
        } else {
            (2 * t + f, f)
        };

        t = next_t;
        f = next_f;
    }

    println!("{}", t);
}
