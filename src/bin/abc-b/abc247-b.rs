use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }

    let is_ok = |i: usize| {
        if (0..n)
            .filter(|&j| j != i)
            .all(|j| st[i].0 != st[j].0 && st[i].0 != st[j].1)
        {
            return true;
        }

        if (0..n)
            .filter(|&j| j != i)
            .all(|j| st[i].1 != st[j].0 && st[i].1 != st[j].1)
        {
            return true;
        }

        false
    };

    let ans = (0..n).all(|i| is_ok(i));
    println!("{}", if ans { "Yes" } else { "No" });
}
