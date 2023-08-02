use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
        mut bb: [usize; n],
        mut cc: [usize; n],
    }

    aa.sort_unstable();
    bb.sort_unstable();
    cc.sort_unstable();

    let mut ans = 0;
    let mut a_idx = 0;
    let mut b_idx = 0;
    let mut c_idx = 0;
    while a_idx < n && b_idx < n && c_idx < n {
        while b_idx < n && bb[b_idx] <= aa[a_idx] {
            b_idx += 1;
        }
        if b_idx == n {
            break;
        }

        while c_idx < n && cc[c_idx] <= bb[b_idx] {
            c_idx += 1;
        }
        if c_idx == n {
            break;
        }

        ans += 1;
        a_idx += 1;
        b_idx += 1;
        c_idx += 1;
    }

    println!("{}", ans);
}
