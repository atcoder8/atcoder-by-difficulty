use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let digits = s
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    let mut ans = 0_usize;
    let mut sum = 0_usize;
    let mut ten = 1_usize;
    let mut counts = vec![0_usize; 2019];
    counts[0] = 1;

    for &d in digits.iter().rev() {
        sum = (sum + ten * d) % 2019;
        ans += counts[sum];
        ten = (10 * ten) % 2019;
        counts[sum] += 1;
    }

    println!("{}", ans);
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use rand::Rng;

    /// Input data type.
    type Input = Vec<usize>;

    /// Output data type.
    type Output = usize;

    /// Perform the specified number of tests.
    #[test]
    fn test() {
        const NUMBER_OF_TESTS: usize = 1000;

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator();
            let expected_output = expected(input.clone());
            let actual_output = solve(input.clone());

            assert_eq!(
                expected_output, actual_output,
                "
Wrong Answer on Test #{}

[Input]
{:?}

[Expected output]
{:?}

[Actual output]
{:?}
",
                test_case_index, input, expected_output, actual_output
            );
        }
    }

    /// Generate test cases.
    pub fn generator() -> Input {
        let mut rng = rand::thread_rng();

        let digit_num = rng.gen_range(1, 100);

        (0..digit_num).map(|_| rng.gen_range(1, 10)).collect()
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let digits = input;
        let n = digits.len();

        let mut ans = 0;

        for i in 0..n {
            let mut x = 0;

            for j in i..n {
                x = (10 * x + digits[j]) % 2019;

                if x == 0 {
                    ans += 1;
                }
            }
        }

        ans
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        let digits = input;

        let mut ans = 0_usize;
        let mut sum = 0_usize;
        let mut ten = 1_usize;
        let mut counts = vec![0_usize; 2019];
        counts[0] = 1;

        for &d in digits.iter().rev() {
            sum = (sum + ten * d) % 2019;
            ans += counts[sum];
            ten = (10 * ten) % 2019;
            counts[sum] += 1;
        }

        ans
    }
}
