use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (_n, k): (usize, usize),
        prizes: [usize; 3],
        t: String,
    }

    let win_hands = t.chars().map(|hand| win_hand(encoding(hand))).collect_vec();

    let calc_score = |start: usize| {
        let mut win = prizes[win_hands[start]];
        let mut lose = 0;

        for (&prev_win_hand, &curt_win_hand) in win_hands[start..].iter().step_by(k).tuple_windows()
        {
            let prize = prizes[curt_win_hand];
            let prev_max = win.max(lose);

            let next_win = if prev_win_hand == curt_win_hand {
                lose + prize
            } else {
                prev_max + prize
            };

            (win, lose) = (next_win, prev_max);
        }

        win.max(lose)
    };

    let ans = (0..k).map(calc_score).sum::<usize>();
    println!("{}", ans);
}

fn encoding(hand: char) -> usize {
    match hand {
        'r' => 0,
        's' => 1,
        'p' => 2,
        _ => unreachable!(),
    }
}

fn win_hand(hand: usize) -> usize {
    (hand + 2) % 3
}
