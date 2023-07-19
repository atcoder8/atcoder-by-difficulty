use itertools::Itertools;
use maplit::hashset;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut xy: [(usize, usize); m],
    }

    let mut pawns = hashset! {n};
    for (_, group) in &xy.iter().sorted().group_by(|x| x.0) {
        let mut remove_pawns = hashset! {};
        let mut add_pawns = hashset! {};

        for &(_, y) in group {
            if pawns.contains(&y) {
                remove_pawns.insert(y);
            }

            if (y > 0 && pawns.contains(&(y - 1))) || (y < 2 * n && pawns.contains(&(y + 1))) {
                add_pawns.insert(y);
            }
        }

        for remove_pawn in &remove_pawns {
            pawns.remove(remove_pawn);
        }

        pawns.extend(add_pawns.into_iter())
    }

    println!("{}", pawns.len());
}
