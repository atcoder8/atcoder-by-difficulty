use itertools::{enumerate, Itertools};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

use crate::fenwick_tree::FenwickTree;

fn main() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
    }

    let c2u = |c: char| (c as u8 - b'a') as usize;

    let mut seq = s.iter().map(|&c| c2u(c)).collect_vec();

    let mut counts = vec![0; 26];
    for &c in &seq {
        counts[c] += 1;
    }

    let mut fts = vec![FenwickTree::<usize>::new(n); 26];
    for (i, &c) in enumerate(&seq) {
        fts[c].add(i, 1);
    }

    for _ in 0..q {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                (x, c): (Usize1, char),
            }

            let from = seq[x];
            let to = c2u(c);

            counts[from] -= 1;
            counts[to] += 1;

            fts[from].sub(x, 1);
            fts[to].add(x, 1);

            seq[x] = to;
        } else {
            input! {
                (l, r): (Usize1, usize),
            }

            let ans = (|| {
                let left_c = seq[l];

                let mut ok = l;
                let mut ng = r;
                while ok.abs_diff(ng) > 1 {
                    let mid = (ok + ng) / 2;

                    if seq[mid] == left_c {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }

                if fts[left_c].sum(l..ng) != ng - l {
                    return false;
                }

                let mut pos = ng;
                let mut u = left_c + 1;
                while u < 26 && pos < r {
                    let next_pos = (pos + counts[u]).min(r);

                    if fts[u].sum(pos..next_pos) != next_pos - pos {
                        return false;
                    }

                    pos = next_pos;
                    u += 1;
                }

                pos == r
            })();

            println!("{}", if ans { "Yes" } else { "No" });
        }
    }
}

pub mod fenwick_tree {
    //! Processes the following query in `O(log(n))` time
    //! for a sequence of numbers with `n` elements:
    //! * Update one element
    //! * Calculate the sum of the elements of a range
    //! * Gets the elements of a number sequence.

    use std::ops::{AddAssign, RangeBounds, Sub, SubAssign};

    /// # Examples
    ///
    /// ```
    /// use atcoder8_library::fenwick_tree::FenwickTree;
    ///
    /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
    /// assert_eq!(ft.sum(2..), 11);
    /// ```
    #[derive(Debug, Clone)]
    pub struct FenwickTree<T>(Vec<T>);

    impl<T> From<Vec<T>> for FenwickTree<T>
    where
        T: Default + Clone + AddAssign<T>,
    {
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(2..6), 9);
        /// ```
        fn from(t: Vec<T>) -> Self {
            let mut ft = FenwickTree::new(t.len());
            for (i, x) in t.into_iter().enumerate() {
                ft.add(i, x);
            }

            ft
        }
    }

    impl<T> FenwickTree<T> {
        /// Constructs a `FenwickTree<T>` with `n` elements.
        ///
        /// Each element is initialized with `T::default()`.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::<i32>::new(5);
        /// assert_eq!(ft.sum(..), 0);
        /// ```
        pub fn new(n: usize) -> Self
        where
            T: Default + Clone,
        {
            FenwickTree(vec![T::default(); n])
        }

        /// Add `x` to the `p`-th element.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let mut ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(2..6), 9);
        ///
        /// ft.add(3, 100);
        /// assert_eq!(ft.sum(2..6), 109);
        /// ```
        pub fn add(&mut self, p: usize, x: T)
        where
            T: Clone + AddAssign<T>,
        {
            let FenwickTree(data) = self;
            let n = data.len();

            assert!(p < n);

            let mut p = p + 1;
            while p <= n {
                data[p - 1] += x.clone();
                p += p & p.overflowing_neg().0;
            }
        }

        /// Subtract `x` from the `p`-th element.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let mut ft = FenwickTree::<u32>::from(vec![3, 1, 4, 1, 5, 9, 2]);
        /// assert_eq!(ft.sum(2..6), 19);
        ///
        /// ft.sub(3, 1);
        /// assert_eq!(ft.sum(2..6), 18);
        /// ```
        pub fn sub(&mut self, p: usize, x: T)
        where
            T: Clone + SubAssign<T>,
        {
            let FenwickTree(data) = self;
            let n = data.len();

            assert!(p < n);

            let mut p = p + 1;
            while p <= n {
                data[p - 1] -= x.clone();
                p += p & p.overflowing_neg().0;
            }
        }

        /// Sets `x` to the `p`-th element.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let mut ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(2..6), 9);
        ///
        /// ft.set(3, 100);
        /// assert_eq!(ft.sum(2..6), 108);
        /// ```
        pub fn set(&mut self, p: usize, x: T)
        where
            T: Default + Clone + AddAssign<T> + Sub<T, Output = T> + SubAssign<T>,
        {
            let FenwickTree(data) = self;
            let n = data.len();

            assert!(p < n);

            self.sub(p, self.get(p));
            self.add(p, x);
        }

        /// Compute the sum of the range [0, r).
        fn inner_sum(&self, r: usize) -> T
        where
            T: Default + Clone + AddAssign<T>,
        {
            let mut s = T::default();
            let mut r = r;
            while r > 0 {
                s += self.0[r - 1].clone();
                r -= r & r.wrapping_neg();
            }

            s
        }

        /// Calculate the total of the range.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(..), 13);
        /// assert_eq!(ft.sum(2..), 11);
        /// assert_eq!(ft.sum(..6), 11);
        /// assert_eq!(ft.sum(2..6), 9);
        /// assert_eq!(ft.sum(6..2), 0);
        /// ```
        pub fn sum<R>(&self, rng: R) -> T
        where
            T: Default + Clone + AddAssign<T> + Sub<T, Output = T>,
            R: RangeBounds<usize>,
        {
            let n = self.0.len();

            let l = match rng.start_bound() {
                std::ops::Bound::Included(&start_bound) => start_bound,
                std::ops::Bound::Excluded(&start_bound) => start_bound + 1,
                std::ops::Bound::Unbounded => 0,
            };

            let r = match rng.end_bound() {
                std::ops::Bound::Included(&end_bound) => end_bound + 1,
                std::ops::Bound::Excluded(&end_bound) => end_bound,
                std::ops::Bound::Unbounded => n,
            };

            assert!(l <= n && r <= n);

            if l >= r {
                T::default()
            } else {
                self.inner_sum(r) - self.inner_sum(l)
            }
        }

        /// Returns the value of an element in a sequence of numbers.
        /// Calculate the total of the range.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::from(vec![3, -1, 4, -1, 5]);
        /// assert_eq!(ft.get(2), 4);
        /// ```
        pub fn get(&self, p: usize) -> T
        where
            T: Default + Clone + AddAssign<T> + Sub<T, Output = T>,
        {
            assert!(p < self.0.len());

            self.sum(p..=p)
        }
    }
}
