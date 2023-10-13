use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    println!("{}", solve(aa));
}

fn solve(aa: Vec<usize>) -> usize {
    let n = aa.len();

    let one_counts = (0..30)
        .map(|pos| aa.iter().filter(|&&a| a >> pos & 1 == 1).count())
        .collect_vec();

    let calc_xor_sum = |x: usize| {
        let mut xor_sum = 0;
        for pos in 0..30 {
            if x >> pos & 1 == 0 {
                xor_sum += one_counts[pos] << pos;
            } else {
                xor_sum += (n - one_counts[pos]) << pos;
            }
        }

        xor_sum
    };

    aa.iter()
        .chain(&[0])
        .map(|&x| calc_xor_sum(x))
        .max()
        .unwrap()
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
        const NUMBER_OF_TESTS: usize = 10000;

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

        let n = rng.gen_range(1..=100);
        let aa = (0..n).map(|_| rng.gen_range(0..(1 << 5))).collect();

        aa
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let aa = input;

        aa.iter()
            .map(|&pivot| aa.iter().map(|&a| a ^ pivot).sum::<usize>())
            .max()
            .unwrap()
            .max(aa.iter().sum())
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        let aa = input;

        super::solve(aa)
    }
}
