use hashbag::HashBag;
use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let mut counter = HashBag::<usize>::new();
    counter.insert(0);

    let mut ans = 0;
    let mut sum = 0;

    for &a in &aa[..(k - 1).min(n)] {
        sum = (sum + a - 1) % k;
        ans += counter.contains(&sum);
        counter.insert(sum);
    }

    let mut remove_sum = 0;

    for (i, &a) in enumerate(&aa[(k - 1).min(n)..]) {
        counter.remove(&remove_sum);
        remove_sum = (remove_sum + aa[i] - 1) % k;

        sum = (sum + a - 1) % k;
        ans += counter.contains(&sum);
        counter.insert(sum);
    }

    println!("{}", ans);
}
