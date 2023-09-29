use std::cmp::Reverse;

use proconio::input;

use crate::{fenwick_tree::FenwickTree, unique_position::UniquePosition};

fn main() {
    input! {
        (n, m): (usize, usize),
        mut aaa: [usize; m * n],
    }

    let mut ans = m * (m + 1) / 2 * n * (n - 1) / 2;
    let mut unique_pos = UniquePosition::from(aaa.clone());
    let mut ft: FenwickTree<usize> = FenwickTree::new(n * m);
    for aa in aaa.chunks_mut(m).rev() {
        aa.sort_unstable_by_key(|&a| Reverse(a));

        for a in aa {
            let pos = unique_pos.zip(a);
            ans += ft.sum(..pos);
            ft.add(pos, 1);
        }
    }

    println!("{}", ans);
}

pub mod unique_position {
    #[derive(Debug, Clone)]
    pub struct UniquePosition<T> {
        unique: Vec<T>,
        organized: bool,
    }

    impl<T> Default for UniquePosition<T> {
        fn default() -> Self {
            Self {
                unique: vec![],
                organized: true,
            }
        }
    }

    impl<T> From<Vec<T>> for UniquePosition<T>
    where
        T: Clone + Ord,
    {
        fn from(seq: Vec<T>) -> Self {
            Self {
                unique: seq,
                organized: false,
            }
        }
    }

    impl<T> UniquePosition<T>
    where
        T: Clone + Ord,
    {
        pub fn new() -> Self {
            Self {
                unique: vec![],
                organized: true,
            }
        }

        pub fn push(&mut self, x: T) {
            self.unique.push(x);
            self.organized = false;
        }

        pub fn extend<I>(&mut self, other: I)
        where
            I: IntoIterator<Item = T>,
        {
            self.unique.extend(other);
            self.organized = false;
        }

        fn organize(&mut self) {
            if !self.organized {
                self.unique.sort_unstable();
                self.unique.dedup();
                self.organized = true;
            }
        }

        pub fn zip(&mut self, x: &T) -> usize {
            self.organize();

            self.unique.binary_search(x).unwrap_or_else(|_| {
                panic!("`x` is not included in the sequence.");
            })
        }

        pub fn unzip(&mut self, i: usize) -> &T {
            assert!(
                i < self.unique.len(),
                "{} was specified, but the number of unique elements in the sequence is {}.",
                i,
                self.unique.len()
            );

            self.organize();

            &self.unique[i]
        }

        pub fn unique_len(&mut self) -> usize {
            self.organize();

            self.unique.len()
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
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FenwickTree<T>(Vec<T>)
    where
        T: Clone + Default + Sub<T, Output = T> + AddAssign<T>;

    impl<T> From<Vec<T>> for FenwickTree<T>
    where
        T: Clone + Default + Sub<T, Output = T> + AddAssign<T>,
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

    impl<T> FenwickTree<T>
    where
        T: Clone + Default + Sub<T, Output = T> + AddAssign<T>,
    {
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
        pub fn new(n: usize) -> Self {
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
        pub fn add(&mut self, p: usize, x: T) {
            let FenwickTree(data) = self;
            let n = data.len();

            assert!(p < n);

            let mut p = p + 1;
            while p <= n {
                data[p - 1] += x.clone();
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
        pub fn set(&mut self, p: usize, x: T) {
            let FenwickTree(data) = self;
            let n = data.len();

            assert!(p < n);

            let t = x - self.get(p);
            self.add(p, t);
        }

        /// Compute the sum of the range [0, r).
        fn inner_sum(&self, r: usize) -> T {
            let mut r = r;
            let mut s = T::default();
            while r > 0 {
                s += self.0[r - 1].clone();
                r -= r & r.overflowing_neg().0;
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
        pub fn get(&self, p: usize) -> T {
            assert!(p < self.0.len());

            self.sum(p..=p)
        }
    }

    impl<T> FenwickTree<T>
    where
        T: Clone + Default + Sub<T, Output = T> + AddAssign<T> + SubAssign<T>,
    {
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
        pub fn sub(&mut self, p: usize, x: T) {
            let FenwickTree(data) = self;
            let n = data.len();

            assert!(p < n);

            let mut p = p + 1;
            while p <= n {
                data[p - 1] -= x.clone();
                p += p & p.overflowing_neg().0;
            }
        }
    }
}
