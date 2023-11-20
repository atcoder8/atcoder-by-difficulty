use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut xor = 0;
    for _ in 0..n {
        input! {
            shape: [usize; 3],
            m: usize,
            coords: [[usize; 3]; m],
        }

        for axis in 0..3 {
            let min = coords.iter().map(|coord| coord[axis]).min().unwrap();
            xor ^= min;

            let max = coords.iter().map(|coord| coord[axis]).max().unwrap();
            xor ^= shape[axis] - 1 - max;
        }
    }

    println!("{}", if xor != 0 { "WIN" } else { "LOSE" });
}
