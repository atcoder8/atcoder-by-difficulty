use itertools::join;
use proconio::input;

const MOD: usize = 200;

fn main() {
    if let Some((bb, cc)) = solve() {
        println!("Yes");
        println!("{} {}", bb.len(), join(bb, " "));
        println!("{} {}", cc.len(), join(cc, " "));
    } else {
        println!("No");
    }
}

fn solve() -> Option<(Vec<usize>, Vec<usize>)> {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    let calc_rem = |mask: usize| {
        (0..n)
            .take_while(|&i| mask >> i != 0)
            .filter(|&i| (mask >> i) & 1 == 1)
            .map(|i| aa[i])
            .sum::<usize>()
            % MOD
    };

    let reconstruct = |mask: usize| -> Vec<usize> {
        (0..n)
            .take_while(|&i| mask >> i != 0)
            .filter(|&i| (mask >> i) & 1 == 1)
            .map(|i| i + 1)
            .collect()
    };

    let mut rem_to_mask = vec![None; MOD];
    for mask in 1..(1_usize << n.min(8)) {
        let rem = calc_rem(mask);

        if let Some(prev_mask) = rem_to_mask[rem] {
            let bb = reconstruct(prev_mask);
            let cc = reconstruct(mask);

            return Some((bb, cc));
        }

        rem_to_mask[rem] = Some(mask);
    }

    None
}
