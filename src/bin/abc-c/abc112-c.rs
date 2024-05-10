use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xyh: [(usize, usize, usize); n],
    }

    let is_ok = |cx: usize, cy: usize, ch: usize| {
        xyh.iter().all(|&(x, y, h)| {
            let dx = x.abs_diff(cx);
            let dy = y.abs_diff(cy);

            ch.saturating_sub(dx + dy) == h
        })
    };

    let calc_ch = |cx: usize, cy: usize| {
        let ch = {
            let (x, y, h) = *xyh.iter().find(|v| v.2 != 0).unwrap();
            let diff_x = x.abs_diff(cx);
            let diff_y = y.abs_diff(cy);

            h + diff_x + diff_y
        };

        if !is_ok(cx, cy, ch) {
            return None;
        }

        return Some(ch);
    };

    let (cx, cy, ch) = iproduct!(0..=100, 0..=100)
        .find_map(|(cx, cy)| {
            let ch = calc_ch(cx, cy)?;
            Some((cx, cy, ch))
        })
        .unwrap();
    println!("{} {} {}", cx, cy, ch);
}
