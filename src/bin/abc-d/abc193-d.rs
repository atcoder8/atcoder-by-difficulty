use proconio::{input, marker::Chars};

fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    }

    let mut takahashi_card_counts = vec![0; 10];
    for c in s[..4].iter() {
        let card = c.to_digit(10).unwrap() as usize;
        takahashi_card_counts[card] += 1;
    }

    let mut aoki_card_counts = vec![0; 10];
    for c in t[..4].iter() {
        let card = c.to_digit(10).unwrap() as usize;
        aoki_card_counts[card] += 1;
    }

    let mut rem_counts = vec![k; 10];
    rem_counts[0] = 0;
    for i in 1..=9 {
        rem_counts[i] -= takahashi_card_counts[i] + aoki_card_counts[i];
    }

    let mut ans = 0.0;
    for i in 1..=9 {
        if rem_counts[i] == 0 {
            continue;
        }

        let p1 = calc_prob(&rem_counts, i);

        rem_counts[i] -= 1;
        takahashi_card_counts[i] += 1;

        for j in 1..=9 {
            if rem_counts[j] == 0 {
                continue;
            }

            let p2 = calc_prob(&rem_counts, j);

            aoki_card_counts[j] += 1;

            if calc_score(&takahashi_card_counts) > calc_score(&aoki_card_counts) {
                ans += p1 * p2;
            }

            aoki_card_counts[j] -= 1;
        }

        rem_counts[i] += 1;
        takahashi_card_counts[i] -= 1;
    }

    println!("{}", ans);
}

fn calc_prob(rem_counts: &Vec<usize>, select_idx: usize) -> f64 {
    let sum: usize = rem_counts.iter().sum();

    rem_counts[select_idx] as f64 / sum as f64
}

fn calc_score(card_counts: &[usize]) -> usize {
    card_counts
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, &cnt)| i * 10_usize.pow(cnt as u32))
        .sum()
}
