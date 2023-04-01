use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    let xy_set: HashSet<(usize, usize)> = xy.iter().cloned().collect();

    let mut ans = 0;

    for i in 0..n {
        let (x1, y1) = xy[i];

        for j in (i + 1)..n {
            let (x2, y2) = xy[j];

            if x1 == x2 || y1 == y2 {
                continue;
            }

            if xy_set.contains(&(x1, y2)) && xy_set.contains(&(x2, y1)) {
                ans += 1;
            }
        }
    }

    ans /= 2;

    println!("{}", ans);
}
