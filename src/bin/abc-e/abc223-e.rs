use itertools::Itertools;
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (x, y): (usize, usize),
        areas: [usize; 3],
    }

    let mut stack = areas
        .into_iter()
        .permutations(3)
        .map(|perm| (x, y, perm))
        .collect_vec();
    while let Some((h, w, mut areas)) = stack.pop() {
        let Some(area) = areas.pop() else { return true; };

        if h == 0 || w == 0 {
            continue;
        }

        let row_num = (area + w - 1) / w;
        if row_num <= h {
            stack.push((h - row_num, w, areas.clone()));
        }

        let col_num = (area + h - 1) / h;
        if col_num <= w {
            stack.push((h, w - col_num, areas));
        }
    }

    false
}
