use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        mut aaa: [[Usize1]; m],
    }

    let mut color_to_pipe: Vec<Option<usize>> = vec![None; n];
    let mut stack = aaa
        .iter_mut()
        .enumerate()
        .map(|(i, aa)| (aa.pop().unwrap(), i))
        .collect_vec();

    while let Some((color, pipe)) = stack.pop() {
        if let Some(other_pipe) = color_to_pipe[color] {
            if let Some(next_color) = aaa[other_pipe].pop() {
                stack.push((next_color, other_pipe));
            }

            if let Some(next_color) = aaa[pipe].pop() {
                stack.push((next_color, pipe));
            }
        } else {
            color_to_pipe[color] = Some(pipe);
        }
    }

    let ans = aaa.iter().all(|aa| aa.is_empty());
    println!("{}", if ans { "Yes" } else { "No" });
}
