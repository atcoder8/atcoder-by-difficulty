use itertools::{join, Itertools};
use proconio::input;

use crate::atcoder8_library::eratosthenes_sieve::EratosthenesSieve;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let sieve = EratosthenesSieve::new(100_000);
    let mut primes = vec![];
    for &a in &aa {
        let factors = sieve.prime_factorization(a);
        primes.extend(factors.iter().map(|x| x.0));
    }
    primes.sort_unstable();
    primes.dedup();

    let mut coprime_flags = vec![true; m + 1];
    for &p in &primes {
        for i in 1..=(m / p) {
            coprime_flags[p * i] = false;
        }
    }

    let ans = (1..=m).filter(|&i| coprime_flags[i]).collect_vec();
    println!("{}\n{}", ans.len(), join(ans, "\n"));
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
}