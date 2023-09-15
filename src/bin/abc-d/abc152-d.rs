use proconio::input;

fn main() {
    println!("{}", solve());
}

fn usize_to_digits(n: usize) -> Vec<usize> {
    let mut digits = vec![];
    let mut n = n;

    while n != 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits
}

fn get_both_ends(n: usize) -> (usize, usize) {
    let mut head = n;
    while head >= 10 {
        head /= 10;
    }

    (head, n % 10)
}

fn solve() -> usize {
    input! {
        n: usize,
    }

    if n < 10 {
        return n;
    }

    let digits = usize_to_digits(n);
    let digit_num = digits.len();

    let (head_n, tail_n) = get_both_ends(n);

    let less_digit_comb_num = (10_usize.pow(digit_num as u32 - 2) - 1) / 9;
    let less_head_comb_num = 10_usize.pow(digit_num as u32 - 2);
    let equal_head_comb_num = n / 10 - head_n * 10_usize.pow(digit_num as u32 - 2);

    (1..=n)
        .map(|i| {
            let (tail, head) = get_both_ends(i);

            if head == 0 {
                return 0;
            }

            less_digit_comb_num
                + (head == tail) as usize
                + if head < head_n {
                    less_head_comb_num
                } else if head > head_n {
                    0
                } else {
                    equal_head_comb_num + (tail <= tail_n) as usize
                }
        })
        .sum()
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use rand::Rng;

    use super::*;

    /// Input data type.
    type Input = usize;

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

        let n = rng.gen_range(1..100);

        n
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let n = input;

        let mut ans = 0;

        for i in 1..=n {
            let (head_i, tail_i) = get_both_ends(i);

            for j in 1..=n {
                let (head_j, tail_j) = get_both_ends(j);

                if head_i == tail_j && tail_i == head_j {
                    ans += 1;
                }
            }
        }

        ans
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        let n = input;

        if n < 10 {
            return n;
        }

        let digits = usize_to_digits(n);
        let digit_num = digits.len();

        let (head_n, tail_n) = get_both_ends(n);

        let less_digit_comb_num = (10_usize.pow(digit_num as u32 - 2) - 1) / 9;
        let less_head_comb_num = 10_usize.pow(digit_num as u32 - 2);
        let equal_head_comb_num = n / 10 - head_n * 10_usize.pow(digit_num as u32 - 2);

        (1..=n)
            .map(|i| {
                let (tail, head) = get_both_ends(i);

                if head == 0 {
                    return 0;
                }

                less_digit_comb_num
                    + (head == tail) as usize
                    + if head < head_n {
                        less_head_comb_num
                    } else if head > head_n {
                        0
                    } else {
                        equal_head_comb_num + (tail <= tail_n) as usize
                    }
            })
            .sum()
    }
}
