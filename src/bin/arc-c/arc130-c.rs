use itertools::iproduct;
use proconio::input;

const RADIX: usize = 10;

fn main() {
    let (a, b) = solve();
    println!("{a}\n{b}");
}

fn solve() -> (String, String) {
    input! {
        a: String,
        b: String,
    }

    let mut counts_a = vec![0; RADIX];
    for c in a.chars() {
        counts_a[char_to_usize(c)] += 1;
    }
    let mut counts_b = vec![0; RADIX];
    for c in b.chars() {
        counts_b[char_to_usize(c)] += 1;
    }

    let create_sorted_ab = |a0: usize| {
        let mut counts_a = counts_a.clone();
        let mut counts_b = counts_b.clone();

        if counts_a[a0] == 0 {
            return None;
        }

        let Some(b0) = (RADIX - a0..RADIX).find(|&i| counts_b[i] != 0) else { return None; };

        let mut sorted_a = vec![usize_to_char(a0)];
        let mut sorted_b = vec![usize_to_char(b0)];

        counts_a[a0] -= 1;
        counts_b[b0] -= 1;

        let mut carry_cnt = 0;
        while let Some((ai, bi)) = select_min_pair(&counts_a, &counts_b) {
            sorted_a.push(usize_to_char(ai));
            sorted_b.push(usize_to_char(bi));

            counts_a[ai] -= 1;
            counts_b[bi] -= 1;

            carry_cnt += 1;
        }

        if counts_a.iter().all(|&cnt| cnt == 0) {
            carry_cnt += counts_b[RADIX - 1];
        } else if counts_b.iter().all(|&cnt| cnt == 0) {
            carry_cnt += counts_a[RADIX - 1];
        }

        for i in (1..RADIX).rev() {
            sorted_a.append(&mut vec![usize_to_char(i); counts_a[i]]);
            sorted_b.append(&mut vec![usize_to_char(i); counts_b[i]]);
        }

        sorted_a.reverse();
        sorted_b.reverse();

        Some((sorted_a, sorted_b, carry_cnt))
    };

    let best_sorted_ab = (1..RADIX)
        .filter_map(|a0: usize| create_sorted_ab(a0))
        .max_by_key(|x| x.2);

    match best_sorted_ab {
        Some((sorted_a, sorted_b, _)) => (sorted_a.iter().collect(), sorted_b.iter().collect()),
        None => (a, b),
    }
}

fn char_to_usize(c: char) -> usize {
    c.to_digit(RADIX as u32).unwrap() as usize
}

fn usize_to_char(n: usize) -> char {
    char::from_digit(n as u32, RADIX as u32).unwrap()
}

fn select_min_pair(counts_a: &[usize], counts_b: &[usize]) -> Option<(usize, usize)> {
    iproduct!(1..RADIX, 1..RADIX)
        .filter(|&(i, j)| counts_a[i] != 0 && counts_b[j] != 0 && i + j >= RADIX - 1)
        .min_by_key(|(i, j)| i + j)
}
