use factorial::Factorial;
use modint::Modint1000000007;
use proconio::input;

type Mint = Modint1000000007;

fn main() {
    input! {
        (n, k): (usize, usize),
    }

    let mut fac = Factorial::<Mint>::new();

    let mut ans = fac.combinations_with_repetition(n, n);
    for empty_num in (k + 1)..n {
        ans -= fac.combinations(n, empty_num)
            * fac.combinations_with_repetition(n - empty_num, empty_num);
    }

    println!("{}", ans.val());
}

pub mod modint {
    use std::ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, ShrAssign, Sub, SubAssign,
    };

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
            "Because `a` and `m` are not prime to each other, \
    there is no multiplicative inverse of `a` with `m` as the modulus.
    gcd(a, m) = {}",
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
        /// Calculate the power of `n` using the iterative squaring method.
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
                /// Create a `Modint` instance that is equivalent to `val`.
                pub fn new<T: RemEuclidU32>(val: T) -> Self {
                    Self {
                        val: val.rem_euclid_u32($modulus),
                    }
                }

                /// Create `x` such that `x * denom` is equivalent to `numer`.
                pub fn frac<T: RemEuclidU32>(numer: T, denom: T) -> Self {
                    Self::new(numer) / Self::new(denom)
                }

                /// Create a `Modint` instance from a value of type `u32`
                /// without any internal modulo operation.
                /// The `val` must be less than the modulus.
                pub fn raw(val: u32) -> Self {
                    Self { val }
                }

                /// Return the remainder as a value of type `u32`.
                /// The value is a non-negative integer less than the modulus.
                pub fn val(&self) -> u32 {
                    self.val
                }

                /// Return the multiplicative inverse.
                pub fn inv(&self) -> Self {
                    Self::new(modinv(self.val, $modulus))
                }

                /// Return a modint instance that is equivalent to 0.
                pub fn zero() -> Self {
                    Self::new(0)
                }

                /// Return `true` only if it is equivalent to 0.
                pub fn equiv_zero(&self) -> bool {
                    self.val == 0
                }

                /// Return `Modint` instance that is equivalent to 1.
                pub fn one() -> Self {
                    Self::new(1)
                }

                /// Return `Modint` instance that is equivalent to 2.
                pub fn two() -> Self {
                    Self::new(2)
                }
            }

            impl<T: RemEuclidU32> From<T> for $modint_type {
                /// Create a `Modint` instance that is equivalent to `val`.
                fn from(val: T) -> Self {
                    Self::new(val)
                }
            }

            impl std::fmt::Display for $modint_type {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", self.val)
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

pub mod factorial {
    use std::ops::{Div, Mul};

    pub struct Factorial<T> {
        fac: Vec<T>,
    }

    impl<T> Default for Factorial<T>
    where
        T: Clone + From<usize> + Mul<Output = T> + Div<Output = T>,
    {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> Factorial<T>
    where
        T: Clone + From<usize> + Mul<Output = T> + Div<Output = T>,
    {
        /// Constructs a new, empty `Factorial<T>`.
        pub fn new() -> Self {
            Self {
                fac: vec![T::from(1)],
            }
        }

        /// Returns the factorial of `n`.
        pub fn factorial(&mut self, n: usize) -> T {
            if self.fac.len() < n + 1 {
                for i in (self.fac.len() - 1)..n {
                    self.fac.push(self.fac[i].clone() * (i + 1).into());
                }
            }
            self.fac[n].clone()
        }

        /// Returns the number of choices when selecting `k` from `n` and arranging them in a row.
        pub fn permutations(&mut self, n: usize, k: usize) -> T {
            if n < k {
                T::from(0)
            } else {
                self.factorial(n) / self.factorial(n - k)
            }
        }

        /// Returns the number of choices to select `k` from `n`.
        pub fn combinations(&mut self, n: usize, k: usize) -> T {
            if n < k {
                T::from(0)
            } else {
                self.factorial(n) / (self.factorial(k) * self.factorial(n - k))
            }
        }

        /// Calculate the number of cases when sample of `k` elements from a set of `n` elements, allowing for duplicates.
        pub fn combinations_with_repetition(&mut self, n: usize, k: usize) -> T {
            if n == 0 {
                if k == 0 {
                    T::from(1)
                } else {
                    T::from(0)
                }
            } else {
                self.combinations(n + k - 1, k)
            }
        }
    }
}
