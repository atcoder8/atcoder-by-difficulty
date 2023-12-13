use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let (a, b, c) = find_triangle_tuples(n);
    let second_partition = if b % 6 == 2 { "2" } else { "1" };
    println!(
        "{}1{}{}{}",
        "7".repeat(a),
        "7".repeat(b),
        second_partition,
        "7".repeat(c)
    );
}

fn calc_triangle_number(n: usize) -> usize {
    n * (n + 1) / 2
}

fn find_triangle_tuples(n: usize) -> (usize, usize, usize) {
    for a in 0.. {
        let tri_a = calc_triangle_number(a);

        if tri_a > n {
            break;
        }

        let rem1 = n - tri_a;
        for b in a.. {
            let tri_b = calc_triangle_number(b);

            if tri_b > rem1 {
                break;
            }

            let rem2 = rem1 - tri_b;

            let mut ok = rem2;
            let mut ng = 0_usize;
            while ok.abs_diff(ng) > 1 {
                let mid = (ok + ng) / 2;

                if calc_triangle_number(mid) >= rem2 {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }

            if calc_triangle_number(ok) == rem2 {
                return (a, b, ok);
            }
        }
    }

    unreachable!()
}
