use proconio::input;
use superslice::Ext;

const DIRS: [&str; 16] = [
    "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW", "NW",
    "NNW",
];

const BOUNDARY_SPEED_PER_SCALE: [usize; 12] =
    [2, 15, 33, 54, 79, 107, 138, 171, 207, 244, 284, 326];

fn main() {
    let (dir, w) = solve();
    println!("{} {}", dir, w);
}

fn solve() -> (&'static str, usize) {
    input! {
        (deg, dis): (usize, usize),
    }

    // 小数第2位を四捨五入した風向[m/s]の10倍
    let wind_speed = (dis + 3) / 6;

    if wind_speed <= 2 {
        return ("C", 0);
    }

    let dir = DIRS[(deg + 112) % 3600 / 225];
    let wind_scale = BOUNDARY_SPEED_PER_SCALE.lower_bound(&wind_speed);

    (dir, wind_scale)
}
