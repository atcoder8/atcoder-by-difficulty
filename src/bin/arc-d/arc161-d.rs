use proconio::input;

fn main() {
    input! {
        (n, d): (usize, usize),
    }

    if d > (n - 1) / 2 {
        println!("No");
        std::process::exit(0);
    }

    println!("Yes");
    for i in 0..n {
        for j in 1..=d {
            println!("{} {}", i + 1, (i + j) % n + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        for n in 3..=16 {
            for d in 1..=(n - 1) / 2 {
                inspect(n, d);
            }
        }
    }

    fn inspect(n: usize, d: usize) {
        for bit in 1..((1 << n) - 1) {
            let mut edge_cnt = 0;
            for i in 0..n {
                for j in 1..=d {
                    edge_cnt += (bit >> i) & (bit >> (i + j) % n) & 1;
                }
            }

            assert!(edge_cnt < n * d, "n = {n}, d = {d}, bit = {bit:b}, edge_cnt = {edge_cnt}");
        }
    }
}
