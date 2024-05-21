use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (a, b, q): (usize, usize, usize),
        ss: [usize; a],
        tt: [usize; b],
        xx: [usize; q],
    }

    let find_min_dist = |x: usize| {
        let (left_s, right_s) = find_both_sides(&ss, x);
        let (left_t, right_t) = find_both_sides(&tt, x);

        let cand_coord_pairs = [
            (left_s, left_t, true),
            (right_s, right_t, true),
            (left_s, right_t, false),
            (right_s, left_t, false),
        ];

        cand_coord_pairs
            .iter()
            .filter_map(|&(shrine_coord, temple_coord, same_dir)| {
                let shrine_coord = shrine_coord?;
                let temple_coord = temple_coord?;

                let dist1 = x.abs_diff(shrine_coord);
                let dist2 = x.abs_diff(temple_coord);

                let total_dist = if same_dir {
                    dist1.max(dist2)
                } else {
                    dist1 + dist2 + dist1.min(dist2)
                };

                Some(total_dist)
            })
            .min()
            .unwrap()
    };

    let ans = xx.iter().map(|&x| find_min_dist(x)).join("\n");
    println!("{}", ans);
}

fn find_both_sides(coords: &[usize], x: usize) -> (Option<usize>, Option<usize>) {
    let upper_idx = coords.upper_bound(&x);
    let left_coord = upper_idx.checked_sub(1).map(|idx| coords[idx]);
    let right_coord = coords.get(upper_idx).cloned();

    (left_coord, right_coord)
}
