use fixedbitset::FixedBitSet;
use itertools::{enumerate, izip, Itertools};
use modint2::Modint;
use num_integer::Roots;
use proconio::{input, marker::Usize1};

use crate::eratosthenes_sieve::EratosthenesSieve;

type Mint = Modint<3>;

const MAX: usize = 10_usize.pow(6);
const SQRT_MAX: usize = 10_usize.pow(3);

fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [usize; n],
        lr: [(Usize1, usize); q],
    }

    let sieve = EratosthenesSieve::new(MAX);

    let small_primes = (2..=SQRT_MAX).filter(|&i| sieve.is_prime(i)).collect_vec();
    let small_prime_num = small_primes.len();
    let mut p_to_i = vec![None; SQRT_MAX + 1];
    for (i, &p) in enumerate(&small_primes) {
        p_to_i[p] = Some(i);
    }

    let prime_factorization = |n: usize| {
        sieve
            .prime_factorization(n)
            .into_iter()
            .map(|(p, e)| (p, Mint::new(e)))
            .collect_vec()
    };

    let mut small_factors = vec![vec![]; n];
    let mut large_factors = vec![None; n];
    for (i, &a) in enumerate(&aa) {
        for (p, e) in prime_factorization(a) {
            if p <= SQRT_MAX {
                small_factors[i].push((p, e));
            } else {
                large_factors[i] = Some(p);
            }
        }
    }

    let mut acc_small_exps = vec![vec![Mint::new(0); small_prime_num]];
    acc_small_exps.reserve(n);
    for factors in &small_factors {
        let mut small_exps = acc_small_exps.last().unwrap().clone();
        for &(p, e) in factors {
            small_exps[p_to_i[p].unwrap()] += e;
        }

        acc_small_exps.push(small_exps);
    }

    let b = (n / q.sqrt()).max(1);

    let mut ilr = enumerate(lr).map(|(i, (l, r))| (i, l, r)).collect_vec();
    ilr.sort_unstable_by_key(|&(_, l, r)| (l / b, r));

    let resize =
        |large_p: usize, large_exps: &mut [Mint], large_extra_num: &mut usize, insert: bool| {
            let exp = &mut large_exps[large_p];

            let prev = exp.rem() == 0;

            if insert {
                *exp += 1;
            } else {
                *exp -= 1;
            }

            let cur = exp.rem() == 0;

            if prev && !cur {
                *large_extra_num += 1;
            } else if !prev && cur {
                *large_extra_num -= 1;
            }
        };

    let mut large_exps = vec![Mint::new(0); MAX + 1];
    let mut large_extra_num = 0;
    let mut left = 0;
    let mut right = 0;

    let mut ans = FixedBitSet::with_capacity(q);
    for &(qi, l, r) in &ilr {
        // Insert
        for i in (l..left).chain(right..r) {
            if let Some(large_p) = large_factors[i] {
                resize(large_p, &mut large_exps, &mut large_extra_num, true);
            }
        }

        // Remove
        for i in (left..l).chain(r..right) {
            if let Some(large_p) = large_factors[i] {
                resize(large_p, &mut large_exps, &mut large_extra_num, false);
            }
        }

        let is_cubic = large_extra_num == 0
            && izip!(&acc_small_exps[l], &acc_small_exps[r]).all(|(&e1, &e2)| (e2 - e1).rem() == 0);

        ans.set(qi, is_cubic);

        left = l;
        right = r;
    }

    for i in 0..q {
        println!("{}", if ans[i] { "Yes" } else { "No" });
    }
}

pub mod eratosthenes_sieve {
    //! Implements the Sieve of Eratosthenes.
    //!
    //! Finds the smallest prime factor of each number placed on the sieve,
    //! so it can perform Prime Factorization as well as Primality Test.

    #[derive(Debug, Clone)]
    pub struct EratosthenesSieve {
        sieve: Vec<usize>,
    }

    impl EratosthenesSieve {
        /// Constructs a Sieve of Eratosthenes.
        ///
        /// # Arguments
        ///
        /// * `upper_limit` - The largest number placed on the sieve.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.prime_factorization(12), vec![(2, 2), (3, 1)]);
        /// ```
        pub fn new(upper_limit: usize) -> Self {
            let mut sieve: Vec<usize> = (0..=upper_limit).collect();

            for p in (2..).take_while(|&i| i * i <= upper_limit) {
                if sieve[p] != p {
                    continue;
                }

                for i in ((p * p)..=upper_limit).step_by(p) {
                    if sieve[i] == i {
                        sieve[i] = p;
                    }
                }
            }

            Self { sieve }
        }

        /// Returns the least prime factor of `n`.
        ///
        /// However, if `n` is `1`, then `1` is returned.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.get_least_prime_factor(1), 1);
        /// assert_eq!(sieve.get_least_prime_factor(2), 2);
        /// assert_eq!(sieve.get_least_prime_factor(6), 2);
        /// assert_eq!(sieve.get_least_prime_factor(11), 11);
        /// assert_eq!(sieve.get_least_prime_factor(27), 3);
        /// ```
        pub fn get_least_prime_factor(&self, n: usize) -> usize {
            assert_ne!(n, 0, "`n` must be at least 1.");

            self.sieve[n]
        }

        /// Determines if `n` is prime.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert!(!sieve.is_prime(1));
        /// assert!(sieve.is_prime(2));
        /// assert!(!sieve.is_prime(6));
        /// assert!(sieve.is_prime(11));
        /// assert!(!sieve.is_prime(27));
        /// ```
        pub fn is_prime(&self, n: usize) -> bool {
            n >= 2 && self.sieve[n] == n
        }

        /// Performs prime factorization of `n`.
        ///
        /// The result of the prime factorization is returned as a
        /// list of prime factor and exponent pairs.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.prime_factorization(1), vec![]);
        /// assert_eq!(sieve.prime_factorization(12), vec![(2, 2), (3, 1)]);
        /// assert_eq!(sieve.prime_factorization(19), vec![(19, 1)]);
        /// assert_eq!(sieve.prime_factorization(27), vec![(3, 3)]);
        /// ```
        pub fn prime_factorization(&self, n: usize) -> Vec<(usize, usize)> {
            assert_ne!(n, 0, "`n` must be at least 1.");

            let mut n = n;

            let mut factors: Vec<(usize, usize)> = vec![];

            while n != 1 {
                let p = self.sieve[n];

                if factors.is_empty() || factors.last().unwrap().0 != p {
                    factors.push((p, 1));
                } else {
                    factors.last_mut().unwrap().1 += 1;
                }

                n /= p;
            }

            factors
        }

        /// Creates a list of positive divisors of `n`.
        ///
        /// The positive divisors are listed in ascending order.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.create_divisor_list(1), vec![1]);
        /// assert_eq!(sieve.create_divisor_list(12), vec![1, 2, 3, 4, 6, 12]);
        /// assert_eq!(sieve.create_divisor_list(19), vec![1, 19]);
        /// assert_eq!(sieve.create_divisor_list(27), vec![1, 3, 9, 27]);
        /// ```
        pub fn create_divisor_list(&self, n: usize) -> Vec<usize> {
            assert_ne!(n, 0, "`n` must be at least 1.");

            let prime_factors = self.prime_factorization(n);
            let divisor_num: usize = prime_factors.iter().map(|&(_, exp)| exp + 1).product();

            let mut divisors = vec![1];
            divisors.reserve(divisor_num - 1);

            for (p, e) in prime_factors {
                let mut add_divisors = vec![];
                add_divisors.reserve(divisors.len() * e);
                let mut mul = 1;

                for _ in 1..=e {
                    mul *= p;

                    for &d in divisors.iter() {
                        add_divisors.push(d * mul);
                    }
                }

                divisors.append(&mut add_divisors);
            }

            divisors.sort_unstable();

            divisors
        }

        /// Calculates the number of positive divisors of `n`.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.calc_divisor_num(1), 1);
        /// assert_eq!(sieve.calc_divisor_num(12), 6);
        /// assert_eq!(sieve.calc_divisor_num(19), 2);
        /// assert_eq!(sieve.calc_divisor_num(27), 4);
        /// ```
        pub fn calc_divisor_num(&self, n: usize) -> usize {
            assert_ne!(n, 0, "`n` must be at least 1.");

            let mut n = n;

            let mut divisor_num = 1;
            let mut cur_p = None;
            let mut cur_exp = 0;

            while n != 1 {
                let p = self.sieve[n];

                if Some(p) == cur_p {
                    cur_exp += 1;
                } else {
                    divisor_num *= cur_exp + 1;

                    cur_p = Some(p);
                    cur_exp = 1;
                }

                n /= p;
            }

            divisor_num *= cur_exp + 1;

            divisor_num
        }
    }
}

pub mod modint2 {
    //! This module implements modular arithmetic.

    use std::{
        iter::{Product, Sum},
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    };

    type InnerType = u32;

    /// Returns `x` such that `a * x` is equivalent to `1` with `m` as the modulus.
    fn modinv(a: u32, m: u32) -> u32 {
        let (mut a, mut b, mut s, mut t) = (a as i64, m as i64, 1, 0);
        while b != 0 {
            let q = a / b;
            a -= q * b;
            std::mem::swap(&mut a, &mut b);
            s -= q * t;
            std::mem::swap(&mut s, &mut t);
        }

        assert_eq!(
            a.abs(),
            1,
            "\
    There is no multiplicative inverse of `a` with `m` as the modulus, \
    because `a` and `m` are not prime to each other (gcd(a, m) = {}).",
            a.abs()
        );

        ((s % m as i64 + m as i64) % m as i64) as u32
    }

    pub trait Reminder {
        /// Returns the remainder divided by `modulus`.
        fn reminder(self, modulus: InnerType) -> InnerType;
    }

    macro_rules! impl_reminder_for_small_unsigned_int {
        ($($unsigned_small_int: tt), *) => {
            $(
                impl Reminder for $unsigned_small_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        self as InnerType % modulus
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `u8`, `u16` and `u32`.
    impl_reminder_for_small_unsigned_int!(u8, u16, u32);

    macro_rules! impl_reminder_for_large_unsigned_int {
        ($($unsigned_large_int: tt), *) => {
            $(
                impl Reminder for $unsigned_large_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        (self % modulus as Self) as InnerType
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `usize`, `u64` and `u128`.
    impl_reminder_for_large_unsigned_int!(usize, u64, u128);

    macro_rules! impl_reminder_for_small_signed_int {
        ($($signed_small_int: tt), *) => {
            $(
                impl Reminder for $signed_small_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        (self as i32 % modulus as i32 + modulus as i32) as InnerType % modulus
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `i8`, `i16` and `i32`.
    impl_reminder_for_small_signed_int!(i8, i16, i32);

    macro_rules! impl_reminder_for_large_signed_int {
        ($($signed_large_int: tt), *) => {
            $(
                impl Reminder for $signed_large_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        (self % modulus as Self + modulus as Self) as InnerType % modulus
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `isize`, `i64` and `i128`.
    impl_reminder_for_large_signed_int!(isize, i64, i128);

    /// Structure for modular arithmetic.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modint<const MODULUS: InnerType> {
        rem: InnerType,
    }

    impl<const MODULUS: InnerType> std::fmt::Display for Modint<MODULUS> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.rem)
        }
    }

    impl<const MODULUS: InnerType> Default for Modint<MODULUS> {
        /// Returns a `Modint` instance equivalent to `0`.
        fn default() -> Self {
            Self::raw(0)
        }
    }

    impl<T, const MODULUS: InnerType> From<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn from(value: T) -> Self {
            Self::new(value)
        }
    }

    impl<const MODULUS: InnerType> Add<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        fn add(self, rhs: Modint<MODULUS>) -> Self::Output {
            Self::raw((self.rem + rhs.rem) % MODULUS)
        }
    }

    impl<const MODULUS: InnerType> Sub<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        fn sub(self, rhs: Modint<MODULUS>) -> Self::Output {
            Self::raw((self.rem + MODULUS - rhs.rem) % MODULUS)
        }
    }

    impl<const MODULUS: InnerType> Mul<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        fn mul(self, rhs: Modint<MODULUS>) -> Self::Output {
            Self::raw((self.rem as u64 * rhs.rem as u64 % MODULUS as u64) as InnerType)
        }
    }

    impl<const MODULUS: InnerType> Div<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        #[allow(clippy::suspicious_arithmetic_impl)]
        fn div(self, rhs: Modint<MODULUS>) -> Self::Output {
            self * rhs.inv()
        }
    }

    impl<const MODULUS: InnerType> Neg for Modint<MODULUS> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::raw((MODULUS - self.rem) % MODULUS)
        }
    }

    impl<const MODULUS: InnerType> AddAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn add_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self + rhs;
        }
    }

    impl<const MODULUS: InnerType> SubAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn sub_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self - rhs;
        }
    }

    impl<const MODULUS: InnerType> MulAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn mul_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self * rhs;
        }
    }

    impl<const MODULUS: InnerType> DivAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn div_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self / rhs;
        }
    }

    impl<const MODULUS: InnerType, T> Add<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn add(self, rhs: T) -> Self::Output {
            self + Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> Sub<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn sub(self, rhs: T) -> Self::Output {
            self - Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> Mul<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn mul(self, rhs: T) -> Self::Output {
            self * Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> Div<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn div(self, rhs: T) -> Self::Output {
            self / Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> AddAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn add_assign(&mut self, rhs: T) {
            *self += Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType, T> SubAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn sub_assign(&mut self, rhs: T) {
            *self -= Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType, T> MulAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn mul_assign(&mut self, rhs: T) {
            *self *= Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType, T> DivAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn div_assign(&mut self, rhs: T) {
            *self /= Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType> Sum<Modint<MODULUS>> for Modint<MODULUS> {
        fn sum<I: Iterator<Item = Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(0), |acc, x| acc + x)
        }
    }

    impl<'a, const MODULUS: InnerType> Sum<&'a Modint<MODULUS>> for Modint<MODULUS> {
        fn sum<I: Iterator<Item = &'a Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(0), |acc, &x| acc + x)
        }
    }

    impl<const MODULUS: InnerType> Product<Modint<MODULUS>> for Modint<MODULUS> {
        fn product<I: Iterator<Item = Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(1), |acc, x| acc * x)
        }
    }

    impl<'a, const MODULUS: InnerType> Product<&'a Modint<MODULUS>> for Modint<MODULUS> {
        fn product<I: Iterator<Item = &'a Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(1), |acc, &x| acc * x)
        }
    }

    impl<const MODULUS: InnerType> Modint<MODULUS> {
        /// Returns the modulus.
        pub fn modulus() -> InnerType {
            MODULUS
        }

        /// Returns a `Modint` instance equivalent to `a`.
        pub fn new<T>(a: T) -> Self
        where
            T: Reminder,
        {
            Self {
                rem: a.reminder(MODULUS),
            }
        }

        /// Creates a `Modint` instance from a non-negative integer less than `MODULUS`.
        pub fn raw(a: InnerType) -> Self {
            Self { rem: a }
        }

        /// Returns `x` such that `x * q` is equivalent to `p`.
        pub fn frac<T>(p: T, q: T) -> Self
        where
            T: Reminder,
        {
            Self::new(p) / Self::new(q)
        }

        /// Returns the remainder divided by `MODULUS`.
        /// The returned value is a non-negative integer less than `MODULUS`.
        pub fn rem(self) -> InnerType {
            self.rem
        }

        /// Returns the modular multiplicative inverse.
        pub fn inv(self) -> Self {
            Self {
                rem: modinv(self.rem, MODULUS),
            }
        }

        /// Calculates the power of `exp` using the iterative squaring method.
        pub fn pow<T>(self, exp: T) -> Self
        where
            T: ToExponent,
        {
            let mut ret = Self::new(1);
            let mut mul = self;
            let exp = exp.to_exponent();
            let mut t = exp.abs;

            while t != 0 {
                if t & 1 == 1 {
                    ret *= mul;
                }

                mul *= mul;
                t >>= 1;
            }

            if exp.neg {
                ret = ret.inv();
            }

            ret
        }
    }

    pub struct Exponent {
        neg: bool,
        abs: u128,
    }

    pub trait ToExponent {
        fn to_exponent(self) -> Exponent;
    }

    macro_rules! impl_to_exponent_for_unsigned_int {
        ($($ty: tt), *) => {
            $(
                impl ToExponent for $ty {
                    fn to_exponent(self) -> Exponent {
                        Exponent {
                            neg: false,
                            abs: self as u128,
                        }
                    }
                }
            )*
        };
    }

    impl_to_exponent_for_unsigned_int!(usize, u8, u16, u32, u64, u128);

    macro_rules! impl_to_exponent_for_signed_int {
        ($($ty: tt), *) => {
            $(
                impl ToExponent for $ty {
                    fn to_exponent(self) -> Exponent {
                        Exponent {
                            neg: self.is_negative(),
                            abs: self.abs() as u128,
                        }
                    }
                }
            )*
        };
    }

    impl_to_exponent_for_signed_int!(isize, i8, i16, i32, i64, i128);

    #[derive(Debug, Clone)]
    pub struct Factorial<Modint> {
        /// Upper limit of available factorial argument.
        upper_limit: usize,

        /// List of factorials.
        fac: Vec<Modint>,

        /// List of factorial inverses.
        inv_fac: Vec<Modint>,
    }

    impl<const MODULUS: InnerType> Factorial<Modint<MODULUS>> {
        /// Calculates factorial and its inverse for non-negative integers bellow `upper_limit`.
        pub fn new(upper_limit: usize) -> Self {
            let mut fac = vec![Modint::new(1); upper_limit + 1];
            for i in 0..upper_limit {
                fac[i + 1] = fac[i] * (i + 1);
            }

            let mut inv_fac = vec![fac[upper_limit].inv(); upper_limit + 1];
            for i in (0..upper_limit).rev() {
                inv_fac[i] = inv_fac[i + 1] * (i + 1);
            }

            Self {
                upper_limit,
                fac,
                inv_fac,
            }
        }

        /// Returns the factorial `n`.
        pub fn factorial(&self, n: usize) -> Modint<MODULUS> {
            assert!(
                n <= self.upper_limit,
                "The maximum number of available factorial arguments has been exceeded."
            );

            self.fac[n]
        }

        /// Returns the inverse of the factorial of `n`.
        pub fn inverse_factorial(&self, n: usize) -> Modint<MODULUS> {
            assert!(
                n <= self.upper_limit,
                "The maximum number of available factorial arguments has been exceeded."
            );

            self.inv_fac[n]
        }

        /// Calculates the number of ways to select and arrange `k` objects from `n` unique objects.
        pub fn permutations(&self, n: usize, k: usize) -> Modint<MODULUS> {
            if n >= k {
                self.factorial(n) * self.inverse_factorial(n - k)
            } else {
                Modint::new(0)
            }
        }

        /// Calculates the number of ways to select `k` objects from `n` unique objects.
        pub fn combinations(&self, n: usize, k: usize) -> Modint<MODULUS> {
            if n >= k {
                self.factorial(n) * self.inverse_factorial(n - k) * self.inverse_factorial(k)
            } else {
                Modint::new(0)
            }
        }

        /// Calculates the number of ways to select `k` objects from `n` unique objects, allowing for duplicates.
        pub fn combinations_with_repetition(&self, n: usize, k: usize) -> Modint<MODULUS> {
            if n == 0 {
                return if k == 0 {
                    Modint::new(1)
                } else {
                    Modint::new(0)
                };
            }

            self.combinations(n + k - 1, k)
        }
    }

    /// The type `Modint` with 1000000007 as the modulus.
    pub type Modint1000000007 = Modint<1000000007>;

    /// The type `Modint` with 998244353 as the modulus.
    pub type Modint998244353 = Modint<998244353>;
}
