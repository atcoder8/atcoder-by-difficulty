use std::{cmp::Ordering, ops::Sub};

use itertools::{chain, Itertools};
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        ab: [(i64, i64); n],
        cd: [(i64, i64); n],
    }

    if n == 1 {
        return true;
    }

    let create_triplets = |coords: &[Coord], base: Coord| {
        let aligned_coords = coords
            .iter()
            .filter(|coord| **coord != base)
            .map(|coord| *coord - base)
            .sorted_unstable_by(Coord::cmp_by_amplitude)
            .collect_vec();

        chain!(&aligned_coords, [&aligned_coords[0]])
            .tuple_windows()
            .map(|(coord1, coord2)| {
                (
                    coord1.square_abs(),
                    coord1.inner_prod(&coord2),
                    coord1.cross_prod(&coord2),
                )
            })
            .collect_vec()
    };

    let coords1 = ab.into_iter().map(|coord| Coord::from(coord)).collect_vec();
    let coords2 = cd.into_iter().map(|coord| Coord::from(coord)).collect_vec();

    let base1 = coords1[0];
    let mut concat_triplets = create_triplets(&coords1, base1);
    concat_triplets.reserve(2 * (n - 1));

    for &base2 in &coords2 {
        let triplets = create_triplets(&coords2, base2);
        concat_triplets.extend(chain!(&triplets, &triplets).cloned());

        let match_lengths = z_algorithm(&concat_triplets);
        if match_lengths[n - 1..].iter().any(|&len| len >= n - 1) {
            return true;
        }

        concat_triplets.truncate(n - 1);
    }

    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord(i64, i64);

impl From<(i64, i64)> for Coord {
    fn from(value: (i64, i64)) -> Self {
        Self(value.0, value.1)
    }
}

impl Sub<Self> for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Coord {
    fn amplitude_sign(&self) -> i8 {
        if self.1 < 0 || self.1 == 0 && self.0 > 0 {
            -1
        } else {
            1
        }
    }

    fn cmp_by_amplitude(&self, other: &Self) -> Ordering {
        let sign1 = self.amplitude_sign();
        let sign2 = other.amplitude_sign();

        if sign1 != sign2 {
            return sign1.cmp(&sign2);
        }

        let Coord(x1, y1) = *self;
        let Coord(x2, y2) = *other;

        match (y1 * x2).cmp(&(y2 * x1)) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.square_abs().cmp(&other.square_abs()),
            Ordering::Greater => Ordering::Greater,
        }
    }

    fn inner_prod(&self, other: &Self) -> i64 {
        self.0 * other.0 + self.1 * other.1
    }

    fn cross_prod(&self, other: &Self) -> i64 {
        self.0 * other.1 - self.1 * other.0
    }

    fn square_abs(&self) -> i64 {
        self.0.pow(2) + self.1.pow(2)
    }
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
