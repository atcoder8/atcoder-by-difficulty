use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        py: [(Usize1, usize); m],
    }

    let mut cities_per_prefecture = vec![vec![]; n];
    for (i, &(p, y)) in enumerate(&py) {
        cities_per_prefecture[p].push((i, y));
    }
    cities_per_prefecture
        .iter_mut()
        .for_each(|cities| cities.sort_unstable_by_key(|v| v.1));

    let mut id_vec = vec![String::new(); m];
    for (p, cities) in enumerate(&cities_per_prefecture) {
        for (x, &(city, _year)) in enumerate(cities) {
            id_vec[city] = format!("{:06}{:06}", p + 1, x + 1);
        }
    }

    println!("{}", id_vec.iter().join("\n"));
}
