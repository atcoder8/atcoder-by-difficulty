use itertools::join;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    let pos = (0..(n - 1)).find(|&i| aa[i] > aa[i + 1]).unwrap_or(n - 1);
    let delete = aa[pos];
    aa.retain(|&a| a != delete);

    println!("{}", join(aa, " "));
}
