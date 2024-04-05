use num::Integer;
use proconio::input;

fn main() {
    input! {
        (walk1, speed1, rest1): (usize, usize, usize),
        (walk2, speed2, rest2): (usize, usize, usize),
        x: usize,
    }

    let calc_dist = |walk: usize, speed: usize, rest: usize| {
        let period = walk + rest;
        let (repeat, rem) = x.div_mod_floor(&period);

        (repeat * walk + rem.min(walk)) * speed
    };

    let dist1 = calc_dist(walk1, speed1, rest1);
    let dist2 = calc_dist(walk2, speed2, rest2);

    let ans = match dist1.cmp(&dist2) {
        std::cmp::Ordering::Less => "Aoki",
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Takahashi",
    };
    println!("{}", ans);
}
