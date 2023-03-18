use atcoder8_library::modint::Modint1000000007;
use proconio::input;

use crate::atcoder8_library::{eratosthenes_sieve::EratosthenesSieve, modint::Pow};

type Mint = Modint1000000007;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let max_a = *aa.iter().max().unwrap();

    let sieve = EratosthenesSieve::new(max_a);

    let mut exponents = vec![0; max_a + 1];
    for &a in &aa {
        for (p, e) in sieve.prime_factorization(a) {
            exponents[p] = exponents[p].max(e);
        }
    }

    let lcm = exponents
        .iter()
        .enumerate()
        .fold(Mint::new(1), |acc, (p, &e)| acc * Mint::new(p).pow(e));

    let ans = aa.iter().fold(Mint::new(0), |acc, &x| acc + lcm / x);
    println!("{}", ans.val());
}

pub mod atcoder8_library {
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
            /// assert_eq!(sieve.create_divisors_list(1), vec![1]);
            /// assert_eq!(sieve.create_divisors_list(12), vec![1, 2, 3, 4, 6, 12]);
            /// assert_eq!(sieve.create_divisors_list(19), vec![1, 19]);
            /// assert_eq!(sieve.create_divisors_list(27), vec![1, 3, 9, 27]);
            /// ```
            pub fn create_divisors_list(&self, n: usize) -> Vec<usize> {
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

    pub mod modint {
        use std::ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, ShrAssign, Sub, SubAssign,
        };

        use num::{One, Zero};

        pub trait RemEuclidU32 {
            fn rem_euclid_u32(self, modulus: u32) -> u32;
        }

        /// Calculate the modular multiplicative inverse of `a` with `m` as modulus.
        pub fn modinv(a: u32, m: u32) -> u32 {
            assert!(m >= 2);

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
                "The inverse does not exist. gcd(a, m) = {}",
                a.abs()
            );

            s %= m as i64;
            if s < 0 {
                s += m as i64;
            }

            s as u32
        }

        /// This macro implements rem_euclid_u32 for signed integer types of 32 bits or less.
        macro_rules! impl_rem_euclid_u32_for_small_signed {
        ($($small_signed_type:tt),*) => {
            $(
                impl RemEuclidU32 for $small_signed_type {
                    fn rem_euclid_u32(self, modulus: u32) -> u32 {
                        let ret = (self as i32) % (modulus as i32);
                        if ret >= 0 {
                            ret as u32
                        } else {
                            (ret + modulus as i32) as u32
                        }
                    }
                }
            )*
        };
    }

        /// This macro implements rem_euclid_u32 for 64-bit signed integer types (including isize).
        macro_rules! impl_rem_euclid_u32_for_large_signed {
        ($($large_signed_type:tt),*) => {
            $(
                impl RemEuclidU32 for $large_signed_type {
                    fn rem_euclid_u32(self, modulus: u32) -> u32 {
                        let ret = (self as i64) % (modulus as i64);
                        if ret >= 0 {
                            ret as u32
                        } else {
                            (ret + modulus as i64) as u32
                        }
                    }
                }
            )*
        };
    }

        /// This macro implements rem_euclid_u32 for unsigned integer types greater than 32 bits.
        macro_rules! impl_rem_euclid_u32_for_small_unsigned {
        ($($small_unsigned_type:tt),*) => {
            $(
                impl RemEuclidU32 for $small_unsigned_type {
                    fn rem_euclid_u32(self, modulus: u32) -> u32 {
                        self as u32 % modulus
                    }
                }
            )*
        };
    }

        /// This macro implements rem_euclid_u32 for 64-bit and larger unsigned integer types (including usize).
        macro_rules! impl_rem_euclid_u32_for_large_unsigned {
        ($($large_unsigned_type:tt),*) => {
            $(
                impl RemEuclidU32 for $large_unsigned_type {
                    fn rem_euclid_u32(self, modulus: u32) -> u32 {
                        (self % modulus as $large_unsigned_type) as u32
                    }
                }
            )*
        };
    }

        // Implement rem_euclid_u32 for signed integer types of 32 bits or less.
        impl_rem_euclid_u32_for_small_signed!(i8, i16, i32);

        // Implement rem_euclid_u32 for 64-bit signed integer types (including isize).
        impl_rem_euclid_u32_for_large_signed!(i64, isize);

        // Implement rem_euclid_u32 for unsigned integer types of 32 bits or more.
        impl_rem_euclid_u32_for_small_unsigned!(u8, u16, u32);

        // Implement rem_euclid_u32 for unsigned integer types (including usize) of 64 bits or more.
        impl_rem_euclid_u32_for_large_unsigned!(u64, u128, usize);

        // Implement rem_euclid_u32 for i128.
        impl RemEuclidU32 for i128 {
            fn rem_euclid_u32(self, modulus: u32) -> u32 {
                let ret = self % (modulus as i128);
                if ret >= 0 {
                    ret as u32
                } else {
                    (ret + modulus as i128) as u32
                }
            }
        }

        pub trait Pow<T: Copy + ShrAssign> {
            fn pow(self, n: T) -> Self;
        }

        /// Macro to overload binary operation with `$modint_type` for each integer type
        macro_rules! impl_ops {
        ($modint_type:tt, $($other_type:tt),*) => {
            $(
                impl Add<$other_type> for $modint_type {
                    type Output = Self;

                    fn add(self, rhs: $other_type) -> Self::Output {
                        self + Self::new(rhs)
                    }
                }

                impl Add<$modint_type> for $other_type {
                    type Output = $modint_type;

                    fn add(self, rhs: $modint_type) -> Self::Output {
                        $modint_type::new(self) + rhs
                    }
                }

                impl Sub<$other_type> for $modint_type {
                    type Output = Self;

                    fn sub(self, rhs: $other_type) -> Self::Output {
                        self - Self::new(rhs)
                    }
                }

                impl Sub<$modint_type> for $other_type {
                    type Output = $modint_type;

                    fn sub(self, rhs: $modint_type) -> Self::Output {
                        $modint_type::new(self) - rhs
                    }
                }

                impl Mul<$other_type> for $modint_type {
                    type Output = Self;

                    fn mul(self, rhs: $other_type) -> Self::Output {
                        self * Self::new(rhs)
                    }
                }

                impl Mul<$modint_type> for $other_type {
                    type Output = $modint_type;

                    fn mul(self, rhs: $modint_type) -> Self::Output {
                        $modint_type::new(self) * rhs
                    }
                }

                impl Div<$other_type> for $modint_type {
                    type Output = Self;

                    fn div(self, rhs: $other_type) -> Self::Output {
                        self / Self::new(rhs)
                    }
                }

                impl Div<$modint_type> for $other_type {
                    type Output = $modint_type;

                    fn div(self, rhs: $modint_type) -> Self::Output {
                        $modint_type::new(self) / rhs
                    }
                }

                impl AddAssign<$other_type> for $modint_type {
                    fn add_assign(&mut self, other: $other_type) {
                        *self = *self + Self::new(other);
                    }
                }

                impl SubAssign<$other_type> for $modint_type {
                    fn sub_assign(&mut self, other: $other_type) {
                        *self = *self - Self::new(other);
                    }
                }

                impl MulAssign<$other_type> for $modint_type {
                    fn mul_assign(&mut self, other: $other_type) {
                        *self = *self * Self::new(other);
                    }
                }

                impl DivAssign<$other_type> for $modint_type {
                    fn div_assign(&mut self, other: $other_type) {
                        *self = *self / Self::new(other);
                    }
                }
            )*
        };
    }

        /// This macro defines powers of Modint for unsigned integer types.
        macro_rules! impl_power_for_unsigned {
        ($modint_type:tt, $($unsigned_type:tt),*) => {
            $(
                impl Pow<$unsigned_type> for $modint_type {
                    fn pow(self, mut n: $unsigned_type) -> Self {
                        let mut ret = Self::new(1);
                        let mut mul = self;
                        while n != 0 {
                            if n & 1 == 1 {
                                ret *= mul;
                            }
                            mul *= mul;
                            n >>= 1;
                        }
                        ret
                    }
                }
            )*
        };
    }

        /// This macro defines powers of Modint for signed integer types of 32 bits or less.
        macro_rules! impl_power_for_small_signed {
        ($modint_type:tt, $($small_signed_type:tt),*) => {
            $(
                impl Pow<$small_signed_type> for $modint_type {
                    fn pow(self, n: $small_signed_type) -> Self {
                        if n >= 0 {
                            self.pow(n as u32)
                        } else {
                            self.pow(-n as u32).inv()
                        }
                    }
                }
            )*
        };
    }

        /// This macro defines the power of Modint for 64-bit signed integer types (including isize).
        macro_rules! impl_power_for_large_signed {
        ($modint_type:tt, $($large_signed_type:tt),*) => {
            $(
                impl Pow<$large_signed_type> for $modint_type {
                    fn pow(self, n: $large_signed_type) -> Self {
                        if n >= 0 {
                            self.pow(n as u64)
                        } else {
                            self.pow(-n as u64).inv()
                        }
                    }
                }
            )*
        };
    }

        /// This macro generates Modint by specifying the type name and modulus.
        macro_rules! generate_modint {
            ($modint_type:tt, $modulus:literal) => {
                #[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq)]
                pub struct $modint_type {
                    val: u32,
                }

                impl $modint_type {
                    const MOD: u32 = $modulus;
                }

                impl $modint_type {
                    pub fn new<T: RemEuclidU32>(val: T) -> Self {
                        Self {
                            val: val.rem_euclid_u32($modulus),
                        }
                    }

                    pub fn frac<T: RemEuclidU32>(numer: T, denom: T) -> Self {
                        Self::new(numer) / Self::new(denom)
                    }

                    pub fn raw(val: u32) -> Self {
                        Self { val }
                    }

                    pub fn val(&self) -> u32 {
                        self.val
                    }

                    pub fn inv(&self) -> Self {
                        Self::new(modinv(self.val, $modulus))
                    }
                }

                impl<T: RemEuclidU32> From<T> for $modint_type {
                    fn from(val: T) -> Self {
                        Self::new(val)
                    }
                }

                impl Add for $modint_type {
                    type Output = Self;

                    fn add(self, rhs: Self) -> Self::Output {
                        Self::new(self.val + rhs.val)
                    }
                }

                impl Sub for $modint_type {
                    type Output = Self;

                    fn sub(self, rhs: Self) -> Self::Output {
                        Self::new(self.val + $modulus - rhs.val)
                    }
                }

                impl Mul for $modint_type {
                    type Output = Self;

                    fn mul(self, rhs: Self) -> Self::Output {
                        Self::new(self.val as u64 * rhs.val as u64)
                    }
                }

                impl Div for $modint_type {
                    type Output = Self;

                    #[allow(clippy::suspicious_arithmetic_impl)]
                    fn div(self, rhs: Self) -> Self::Output {
                        self * rhs.inv()
                    }
                }

                impl AddAssign for $modint_type {
                    fn add_assign(&mut self, other: Self) {
                        *self = *self + other;
                    }
                }

                impl SubAssign for $modint_type {
                    fn sub_assign(&mut self, other: Self) {
                        *self = *self - other;
                    }
                }

                impl MulAssign for $modint_type {
                    fn mul_assign(&mut self, other: Self) {
                        *self = *self * other;
                    }
                }

                impl DivAssign for $modint_type {
                    fn div_assign(&mut self, other: Self) {
                        *self = *self / other;
                    }
                }

                impl Neg for $modint_type {
                    type Output = Self;

                    fn neg(self) -> Self::Output {
                        Self::new(Self::MOD - self.val)
                    }
                }

                impl Zero for $modint_type {
                    fn zero() -> Self {
                        Self::new(0)
                    }

                    fn is_zero(&self) -> bool {
                        self.val == 0
                    }
                }

                impl One for $modint_type {
                    fn one() -> Self {
                        Self::new(1)
                    }
                }

                // Define a binary operation between each integer type and $modint_type.
                impl_ops!(
                    $modint_type,
                    i8,
                    i16,
                    i32,
                    i64,
                    i128,
                    isize,
                    u8,
                    u16,
                    u32,
                    u64,
                    u128,
                    usize
                );

                // Define powers of Modint for unsigned integer types.
                impl_power_for_unsigned!($modint_type, u8, u16, u32, u64, u128, usize);

                // Define powers of Modint for signed integer types of 32 bits or less.
                impl_power_for_small_signed!($modint_type, i8, i16, i32);

                // Define Modint powers for 64-bit signed integer types (including isize).
                impl_power_for_large_signed!($modint_type, i64, isize);

                // Define the power of Modint for 128-bit signed integer types.
                impl Pow<i128> for $modint_type {
                    fn pow(self, n: i128) -> Self {
                        if n >= 0 {
                            self.pow(n as u128)
                        } else {
                            self.pow(-n as u128).inv()
                        }
                    }
                }
            };
        }

        // Define Modint with 998244353 as modulus
        generate_modint!(Modint998244353, 998244353);

        // Define Modint with 1000000007 as modulus
        generate_modint!(Modint1000000007, 1000000007);
    }
}
