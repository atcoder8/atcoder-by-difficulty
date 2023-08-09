use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        xy_per_person: [[(Usize1, usize)]; n],
    }

    let is_ok = |bit: usize| {
        for person in 0..n {
            if (bit >> person) & 1 == 0 {
                continue;
            }

            for &(x, y) in &xy_per_person[person] {
                if (bit >> x) & 1 != y {
                    return false;
                }
            }
        }

        true
    };

    let mut ans = 0;
    for bit in 0..(1 << n) {
        if is_ok(bit) {
            ans = ans.max(bit.count_ones());
        }
    }

    println!("{}", ans);
}
