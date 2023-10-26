use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let mut pairs = vec![];
    let mut idx = 0;
    while idx + 1 < n {
        if aa[idx] == aa[idx + 1] {
            pairs.push(aa[idx]);
            idx += 2;
        } else {
            idx += 1;
        }
    }

    let pair_num = pairs.len();
    if pair_num >= 2 {
        let area = pairs[pair_num - 1] * pairs[pair_num - 2];
        println!("{}", area);
    } else {
        println!("0");
    }
}
