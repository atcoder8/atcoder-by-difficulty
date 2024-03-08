use itertools::Itertools;
use proconio::input;

const RADIX: usize = 10_usize.pow(2);

fn main() {
    input! {
        _w: usize,
    }

    let weights = (0..3)
        .flat_map(|exp| (1..RADIX).map(move |i| RADIX.pow(exp) * i))
        .chain([RADIX.pow(3)])
        .collect_vec();
    println!("{}\n{}", weights.len(), weights.iter().join(" "));
}
