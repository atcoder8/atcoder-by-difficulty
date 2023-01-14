use proconio::input;

fn main() {
    input! {
        (x, y): (usize, usize),
    }

    let groups = vec![vec![1, 3, 5, 7, 8, 10, 12], vec![4, 6, 9, 11], vec![2]];

    let ans = groups
        .iter()
        .any(|group| group.contains(&x) && group.contains(&y));
    println!("{}", if ans { "Yes" } else { "No" });
}
