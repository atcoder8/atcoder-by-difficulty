use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    let ans = rec(&mut vec![vec![vec![None; 100]; 100]; 100], a, b, c);
    println!("{}", ans);
}

fn rec(dp: &mut Vec<Vec<Vec<Option<f64>>>>, x: usize, y: usize, z: usize) -> f64 {
    if x == 100 || y == 100 || z == 100 {
        return 0.0;
    }

    if let Some(exp) = dp[x][y][z] {
        return exp;
    }

    let sum = x + y + z;

    let exp = x as f64 / sum as f64 * rec(dp, x + 1, y, z)
        + y as f64 / sum as f64 * rec(dp, x, y + 1, z)
        + z as f64 / sum as f64 * rec(dp, x, y, z + 1)
        + 1.0;
    dp[x][y][z] = Some(exp);

    exp
}
