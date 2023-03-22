use itertools::Itertools;
use proconio::input;

type Coord = (f64, f64);

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        ab: [(f64, f64); n],
        cd: [(f64, f64); n],
    }

    if n == 1 {
        return true;
    }

    let (center_a, center_b) = calc_center_coord(&ab);
    let shifted_ab = ab
        .iter()
        .map(|&(a, b)| (a - center_a, b - center_b))
        .collect_vec();

    let (center_c, center_d) = calc_center_coord(&cd);
    let shifted_cd = cd
        .iter()
        .map(|&(c, d)| (c - center_c, d - center_d))
        .collect_vec();

    let (a1, b1) = shifted_ab[is_same_coord(shifted_ab[0], (0.0, 0.0)) as usize];
    let rad1 = b1.atan2(a1);

    for &(c1, d1) in &shifted_cd {
        if is_same_coord((c1, d1), (0.0, 0.0)) {
            continue;
        }

        let rad2 = d1.atan2(c1);

        let rotated_ab = shifted_ab
            .iter()
            .map(|&coord| rotate(coord, rad2 - rad1))
            .collect_vec();

        let match_flag = rotated_ab.iter().all(|&coord1| {
            shifted_cd
                .iter()
                .any(|&coord2| is_same_coord(coord1, coord2))
        });

        if match_flag {
            return true;
        }
    }

    false
}

fn calc_center_coord(xy: &Vec<Coord>) -> Coord {
    let n = xy.len();

    let center_x = xy.iter().map(|coord| coord.0).sum::<f64>() / n as f64;
    let center_y = xy.iter().map(|coord| coord.1).sum::<f64>() / n as f64;

    (center_x, center_y)
}

fn is_same_coord(coord1: Coord, coord2: Coord) -> bool {
    let (x1, y1) = coord1;
    let (x2, y2) = coord2;

    (x1 - x2).abs() <= 1e-6 && (y1 - y2).abs() <= 1e-6
}

fn rotate(coord: Coord, rad: f64) -> Coord {
    let (x, y) = coord;
    let (sin, cos) = rad.sin_cos();

    (cos * x - sin * y, sin * x + cos * y)
}
