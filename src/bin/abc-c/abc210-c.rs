use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        cc: [usize; n],
    }

    let (cc, _) = coordinate_compression(&cc);

    let mut counts = vec![0; n];
    let mut candy_variety = 0;
    for &c in cc.iter().take(k) {
        counts[c] += 1;
        if counts[c] == 1 {
            candy_variety += 1;
        }
    }

    let mut ans = candy_variety;
    for i in 0..(n - k) {
        let dec_cnt = &mut counts[cc[i]];
        *dec_cnt -= 1;
        if *dec_cnt == 0 {
            candy_variety -= 1;
        }

        let inc_cnt = &mut counts[cc[i + k]];
        *inc_cnt += 1;
        if *inc_cnt == 1 {
            candy_variety += 1;
        }

        ans = ans.max(candy_variety);
    }

    println!("{}", ans);
}

/// Performs coordinate compression of `seq`.
///
/// The return value is a tuple of `zip` and `unzip`.
/// `zip` is a list of the number of smallest values in the whole sequence for each element.
/// `unzip` is a list of the values that appear in the number sequence in ascending order.
/// The `i`-th element of the original sequence can be restored by `unzip[zip[i]]`.
pub fn coordinate_compression<T>(seq: &[T]) -> (Vec<usize>, Vec<T>)
where
    T: Clone + Ord,
{
    let mut unzip = seq.to_owned();
    unzip.sort_unstable();
    unzip.dedup();

    let zip: Vec<usize> = seq
        .iter()
        .map(|x| unzip.binary_search(x).unwrap())
        .collect();

    (zip, unzip)
}
