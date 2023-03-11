use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", solve());
}

fn solve() -> i64 {
    input! {
        (n, k): (usize, usize),
        pp: [Usize1; n],
        cc: [i64; n],
    }

    let find_max_score = |start_pos: usize| {
        let mut visited = vec![false; n];
        visited[start_pos] = true;
        let mut acc_score = vec![0];
        let mut pos = start_pos;

        loop {
            let next_pos = pp[pos];
            acc_score.push(acc_score.last().unwrap() + cc[next_pos]);

            if visited[next_pos] {
                break;
            }

            visited[next_pos] = true;
            pos = next_pos;
        }

        let cycle_len = acc_score.len() - 1;

        if k <= cycle_len {
            return *acc_score[1..=k].iter().max().unwrap();
        }

        if acc_score[cycle_len] <= 0 {
            return *acc_score[1..].iter().max().unwrap();
        }

        let cycle_score = acc_score[cycle_len];

        let q = k / cycle_len;
        let r = k % cycle_len;

        cycle_score * (q - 1) as i64
            + acc_score[1..]
                .iter()
                .max()
                .unwrap()
                .max(&(cycle_score + acc_score[..=r].iter().max().unwrap()))
    };

    (0..n)
        .map(|start_pos| find_max_score(start_pos))
        .max()
        .unwrap()
}
