use std::io::Write;

use fixedbitset::FixedBitSet;
use itertools::{join, Itertools};

fn main() {
    let (n, k) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let mut bitsets = vec![];

    let shrink_len = k + 1;
    for i in 0..shrink_len {
        let xx = (0..k).map(|j| (i + j) % shrink_len).collect_vec();
        let mut bitset = FixedBitSet::with_capacity(n + 1);
        for &x in &xx {
            bitset.put(x);
        }
        let t = ask_parity(&xx);
        bitset.set(n, t);

        bitsets.push(bitset);
    }

    let mut base_bitset = FixedBitSet::with_capacity(n + 1);
    for i in 0..(k - 1) {
        base_bitset.put(i);
    }
    let mut xx = (0..(k - 1)).collect_vec();
    for i in shrink_len..n {
        let mut bitset = base_bitset.clone();
        bitset.put(i);
        xx.push(i);
        let t = ask_parity(&xx);
        bitset.set(n, t);
        xx.pop();

        bitsets.push(bitset);
    }

    let rank = bit_sweep(&mut bitsets, true);
    assert_eq!(rank, n);

    let aa = (0..n).map(|i| bitsets[i][n] as usize).collect_vec();
    println!("! {}", join(aa, " "));
    std::io::stdout().flush().unwrap();
}

fn ask_parity(xx: &[usize]) -> bool {
    println!("? {}", join(xx.iter().map(|&x| x + 1), " "));
    std::io::stdout().flush().unwrap();

    let t = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<i8>().unwrap()
    };
    if t == -1 {
        std::process::exit(0);
    }

    t == 1
}

pub fn bit_sweep(bitsets: &mut [FixedBitSet], extended: bool) -> usize {
    if bitsets.is_empty() {
        return 0;
    }

    assert!(bitsets.iter().all(|x| x.len() == bitsets[0].len()));

    let rows = bitsets.len();
    let cols = bitsets[0].len() - extended as usize;

    let mut rank = 0;
    for sweep_pos in 0..cols {
        let pivot = (rank..rows).find(|&row| bitsets[row][sweep_pos]);
        if let Some(pivot) = pivot {
            bitsets.swap(rank, pivot);
            let pivot_bitset = bitsets[rank].clone();

            for row in 0..rows {
                if row != rank && bitsets[row][sweep_pos] {
                    bitsets[row].symmetric_difference_with(&pivot_bitset);
                }
            }

            rank += 1;
        }
    }

    rank
}
