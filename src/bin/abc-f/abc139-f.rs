use proconio::input;

use crate::amplitude_sort::amplitude_sort;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
    }

    xy.retain(|&coord| coord != (0, 0));
    let n = xy.len();

    amplitude_sort(&mut xy);

    let mut ans = 0.0;
    for i in 0..n {
        let mut sum_x = 0;
        let mut sum_y = 0;

        for j in 0..n {
            let (x, y) = xy[(i + j) % n];
            sum_x += x;
            sum_y += y;
            let dist = calc_euclid_dist((sum_x, sum_y));
            if dist > ans {
                ans = dist;
            }
        }
    }

    println!("{}", ans);
}

fn calc_euclid_dist(coord: (i64, i64)) -> f64 {
    ((coord.0.pow(2) + coord.1.pow(2)) as f64).sqrt()
}

pub mod amplitude_sort {
    //! Sorts coordinates in ascending order with respect to amplitude.

    use std::cmp::Ordering;

    type Coord = (i64, i64);

    /// Classification of regions on the xy-plane.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Region {
        Origin,
        XAxisPositive,
        QuadrantOne,
        YAxisPositive,
        QuadrantTwo,
        XAxisNegative,
        QuadrantThree,
        YAxisNegative,
        QuadrantFour,
    }

    impl PartialOrd for Region {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if let (Some(self_order), Some(other_order)) = (self.order(), other.order()) {
                self_order.partial_cmp(&other_order)
            } else {
                None
            }
        }
    }

    impl Region {
        fn determine(coord: Coord) -> Self {
            use Region::*;

            let (x, y) = coord;

            match (x == 0, y == 0) {
                (true, true) => return Origin,
                (true, false) => return if y > 0 { YAxisPositive } else { YAxisNegative },
                (false, true) => return if x > 0 { XAxisPositive } else { XAxisNegative },
                (false, false) => {}
            }

            match (x > 0, y > 0) {
                (true, true) => QuadrantOne,
                (false, true) => QuadrantTwo,
                (false, false) => QuadrantThree,
                (true, false) => QuadrantFour,
            }
        }

        fn order(self) -> Option<u8> {
            match self {
                Region::Origin => None,
                Region::XAxisPositive => Some(0),
                Region::QuadrantOne => Some(1),
                Region::YAxisPositive => Some(2),
                Region::QuadrantTwo => Some(3),
                Region::XAxisNegative => Some(4),
                Region::QuadrantThree => Some(5),
                Region::YAxisNegative => Some(6),
                Region::QuadrantFour => Some(7),
            }
        }
    }

    /// Sorts coordinates in ascending order with respect to amplitude.
    ///
    /// Coordinates with equal amplitude are sorted in ascending order with
    /// respect to their distance from the origin.
    ///
    /// Note: If the absolute value of the x- or y-coordinate value
    /// is greater or equal to `2^31` (31st power of 2), an overflow may occur.
    ///
    /// # Panics
    ///
    /// If the coordinate to be sorted contains the origin (0, 0), it will be panic.
    ///
    /// # Examples
    ///
    /// ```
    /// # use atcoder8_library::amplitude_sort::amplitude_sort;
    /// #
    /// let mut coords: Vec<(i64, i64)> = vec![
    ///     (0, -3), (4, -2), (2, -1), (-2, -2), (1, -4), (-2, 1),
    ///     (2, 2), (-2, 0), (-1, -4), (0, -2), (1, 0), (0, 1),
    ///     (-1, 3), (0, 2), (1, 1), (2, 0), (-1, 0), (3, 1),
    /// ];
    /// amplitude_sort(&mut coords);
    ///
    /// assert_eq!(
    ///     coords,
    ///     vec![
    ///         (1, 0), (2, 0), (3, 1), (1, 1), (2, 2), (0, 1),
    ///         (0, 2), (-1, 3), (-2, 1), (-1, 0), (-2, 0), (-2, -2),
    ///         (-1, -4), (0, -2), (0, -3), (1, -4), (2, -1), (4, -2),
    ///     ]
    /// );
    /// ```
    pub fn amplitude_sort(coords: &mut [Coord]) {
        coords.sort_unstable_by(|&coord1, &coord2| {
            match Region::determine(coord1)
                .partial_cmp(&Region::determine(coord2))
                .unwrap()
            {
                Ordering::Equal => {}
                ord => return ord,
            }

            let (x1, y1) = coord1;
            let (x2, y2) = coord2;

            match (y1 * x2).cmp(&(y2 * x1)) {
                Ordering::Equal => {}
                ord => return ord,
            }

            (coord1.0.pow(2) + coord1.1.pow(2)).cmp(&(coord2.0.pow(2) + coord2.1.pow(2)))
        });
    }
}
