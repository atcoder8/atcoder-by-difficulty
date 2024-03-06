use proconio::input;

const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

fn main() {
    input! {
        board: [usize; 9],
        bb: [usize],
    }

    let mut punched_out = vec![false; 9];
    for &b in &bb {
        if let Some(pos) = board.iter().position(|&number| number == b) {
            punched_out[pos] = true;
        }
    }

    let ans = LINES
        .iter()
        .any(|line| line.iter().all(|&idx| punched_out[idx]));
    println!("{}", if ans { "Yes" } else { "No" });
}
