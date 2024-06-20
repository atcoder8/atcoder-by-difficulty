use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans = 0;
    for i in 0..n {
        let z = z_algorithm(&s[i..]);
        for j in 1..z.len() {
            ans = ans.max(j.min(z[j]));
        }
    }

    println!("{}", ans);
}

/// For each non-negative integer `i` less than `|seq|`,
/// find the length of the longest common prefix of `seq` and `seq[i..]`.
pub fn z_algorithm<T>(seq: &[T]) -> Vec<usize>
where
    T: Eq,
{
    if seq.is_empty() {
        return vec![];
    }

    let n = seq.len();

    let mut lengths = vec![0; n];
    lengths[0] = n;

    let mut cursor = 1;
    let mut common_len = 0;
    while cursor < n {
        while cursor + common_len < n && seq[cursor + common_len] == seq[common_len] {
            common_len += 1;
        }

        if common_len == 0 {
            cursor += 1;
            continue;
        }

        lengths[cursor] = common_len;

        let mut shift = 1;
        while shift + lengths[shift] < common_len {
            lengths[cursor + shift] = lengths[shift];
            shift += 1;
        }

        cursor += shift;
        common_len -= shift;
    }

    lengths
}
