use itertools::Itertools;

fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let aa: Vec<_> = {
        let mut line = String::new();
        line.reserve(2 * n - 1);
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.bytes().next().unwrap())
            .collect()
    };

    let flip_num = (0..n).take_while(|&i| aa[i] & 1 == (i & 1) as u8).count();
    let ans = aa[flip_num..].iter().dedup().count() <= flip_num;

    println!("{}", if ans { "Yes" } else { "No" });
}
