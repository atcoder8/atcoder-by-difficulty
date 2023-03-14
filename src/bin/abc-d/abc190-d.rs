use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = (1..)
        .take_while(|&i| i * (i + 1) / 2 <= n)
        .filter(|&elem_num| {
            let ave = n / elem_num;
            let sum = if elem_num % 2 == 1 {
                ave * elem_num
            } else {
                (2 * ave + 1) * elem_num / 2
            };

            sum == n
        })
        .count()
        * 2;

    println!("{}", ans);
}
