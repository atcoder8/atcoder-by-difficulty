use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

use crate::cumulative_sum::CumulativeSum;

fn main() {
    input! {
        (n, m): (usize, usize),
        tx: [(usize, usize); n],
    }

    let mut opened = vec![];
    let mut closed = vec![];
    let mut cutter = vec![];
    for &(t, x) in &tx {
        match t {
            0 => opened.push(x),
            1 => closed.push(x),
            2 => cutter.push(x),
            _ => panic!(),
        }
    }

    let mut heap = BinaryHeap::new();
    for &x in &opened {
        heap.push((x, true));
    }
    for &x in &closed {
        heap.push((x, false));
    }

    opened.sort_unstable_by_key(|&x| Reverse(x));
    closed.sort_unstable_by_key(|&x| Reverse(x));
    cutter.sort_unstable_by_key(|&x| Reverse(x));

    let cumsum_opened = CumulativeSum::from(opened);
    let cumsum_closed = CumulativeSum::from(closed);
    let cumsum_cutter = CumulativeSum::from(cutter);

    let mut ans = 0;
    let mut opened_num = 0;
    let mut closed_num = 0;
    for cutter_num in 0..=cumsum_cutter.len().min(m) {
        let rem = m - cutter_num;
        if rem < opened_num + closed_num {
            break;
        }

        let can_be_opened_num = cumsum_cutter.sum(..cutter_num);
        while closed_num < can_be_opened_num && opened_num + closed_num < rem && !heap.is_empty() {
            let (_, is_opened) = heap.pop().unwrap();
            if is_opened {
                opened_num += 1;
            } else {
                closed_num += 1;
            }
        }

        ans = ans.max(
            cumsum_opened.sum(..(rem - closed_num).min(cumsum_opened.len()))
                + cumsum_closed.sum(..closed_num),
        );
    }

    println!("{}", ans);
}

pub mod cumulative_sum {
    //! Calculates the interval sum of a numerical sequence
    //! in constant time using cumulative sum.

    use std::ops::{Add, RangeBounds, Sub};

    use num::Zero;

    /// Calculates the interval sum of a numerical sequence using cumulative sum.
    ///
    /// # Examples
    /// ```
    /// use atcoder8_library::cumulative_sum::CumulativeSum;
    ///
    /// let cumsum = CumulativeSum::from(vec![3, -1, 4, -1, -5, 9, 2]);
    /// assert_eq!(cumsum.sum(2..6), 7);
    /// ```
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct CumulativeSum<T>
    where
        T: Clone + Add<Output = T> + Sub<Output = T> + Zero,
    {
        /// Cumulative sum of sequence
        cumsum: Vec<T>,
    }

    impl<T> CumulativeSum<T>
    where
        T: Clone + Add<Output = T> + Sub<Output = T> + Zero,
    {
        /// Calculates cumulative sum for an empty sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let cumsum = CumulativeSum::<i32>::new();
        /// assert_eq!(cumsum.sum(..), 0);
        /// ```
        pub fn new() -> Self {
            Self {
                cumsum: vec![T::zero()],
            }
        }

        /// Returns the length of a sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let cumsum = CumulativeSum::from(vec![3, -1, 4]);
        /// assert_eq!(cumsum.len(), 3);
        /// ```
        pub fn len(&self) -> usize {
            self.cumsum.len() - 1
        }

        /// Checks if the sequence is empty.
        ///
        /// # Examples
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let mut cumsum = CumulativeSum::from(vec![3]);
        /// assert!(!cumsum.is_empty());
        /// cumsum.pop();
        /// assert!(cumsum.is_empty());
        /// ```
        pub fn is_empty(&self) -> bool {
            self.len().is_zero()
        }

        /// Adds an element to the end of the sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let mut cumsum = CumulativeSum::from(vec![3, -1, 4]);
        /// assert_eq!(cumsum.sum(1..), 3);
        /// cumsum.push(10);
        /// assert_eq!(cumsum.sum(1..), 13);
        /// ```
        pub fn push(&mut self, x: T) {
            self.cumsum.push(self.cumsum.last().unwrap().clone() + x);
        }

        /// Removes the last element of a sequence and returns the end of the cumulative sum.
        ///
        /// # Panics
        ///
        /// Panics if the sequence is empty.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let mut cumsum = CumulativeSum::from(vec![3, -1, 4]);
        /// assert_eq!(cumsum.sum(1..), 3);
        /// assert_eq!(cumsum.pop(), 6);
        /// assert_eq!(cumsum.sum(1..), -1);
        /// ```
        pub fn pop(&mut self) -> T {
            assert!(!self.is_empty(), "The sequence is empty.");

            self.cumsum.pop().unwrap()
        }

        /// Calculates the sum of intervals.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let seq = vec![3, -1, 4, -1, -5, 9, 2];
        /// let cumsum = CumulativeSum::from(seq.clone());
        ///
        /// assert_eq!(cumsum.sum(..), seq.iter().sum());
        /// assert_eq!(cumsum.sum(2..), seq[2..].iter().sum());
        /// assert_eq!(cumsum.sum(..5), seq[..5].iter().sum());
        /// assert_eq!(cumsum.sum(2..5), seq[2..5].iter().sum());
        /// assert_eq!(cumsum.sum(2..2), 0);
        /// ```
        pub fn sum<R>(&self, rng: R) -> T
        where
            R: RangeBounds<usize>,
        {
            let l = match rng.start_bound() {
                std::ops::Bound::Included(&start_bound) => start_bound,
                std::ops::Bound::Excluded(&start_bound) => start_bound + 1,
                std::ops::Bound::Unbounded => 0,
            };

            let r = match rng.end_bound() {
                std::ops::Bound::Included(&end_bound) => end_bound + 1,
                std::ops::Bound::Excluded(&end_bound) => end_bound,
                std::ops::Bound::Unbounded => self.cumsum.len() - 1,
            };

            if l >= r {
                return T::zero();
            }

            self.cumsum[r].clone() - self.cumsum[l].clone()
        }

        /// Converts the cumulative sum to `Vec<T>`.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::cumulative_sum::CumulativeSum;
        ///
        /// let cumsum = CumulativeSum::from(vec![3, -1, 4]);
        /// assert_eq!(cumsum.to_vec(), vec![0, 3, 2, 6]);
        /// ```
        pub fn to_vec(self) -> Vec<T>
        where
            T: Clone + Add<Output = T> + Sub<Output = T> + Zero,
        {
            self.cumsum
        }
    }

    impl<T> From<Vec<T>> for CumulativeSum<T>
    where
        T: Clone + Add<Output = T> + Sub<Output = T> + Zero,
    {
        fn from(seq: Vec<T>) -> Self {
            let mut accumulate = vec![T::zero()];
            accumulate.reserve(accumulate.len() - 1);
            for (i, x) in seq.into_iter().enumerate() {
                accumulate.push(accumulate[i].clone() + x);
            }
            Self { cumsum: accumulate }
        }
    }
}
