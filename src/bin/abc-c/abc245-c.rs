use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
        bb: [usize; n],
    }

    let mut f1 = true;
    let mut f2 = true;
    for i in 1..n {
        let mut next_f1 = false;
        let mut next_f2 = false;

        if f1 {
            if aa[i].abs_diff(aa[i - 1]) <= k {
                next_f1 = true;
            }

            if bb[i].abs_diff(aa[i - 1]) <= k {
                next_f2 = true;
            }
        }

        if f2 {
            if aa[i].abs_diff(bb[i - 1]) <= k {
                next_f1 = true;
            }

            if bb[i].abs_diff(bb[i - 1]) <= k {
                next_f2 = true;
            }
        }

        f1 = next_f1;
        f2 = next_f2;
    }

    println!("{}", if f1 || f2 { "Yes" } else { "No" });
}
