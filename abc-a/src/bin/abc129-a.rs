use proconio::input;

fn main() {
    input! {
        mut aa: [usize; 3],
    }

    aa.sort_unstable();

    println!("{}", aa[0] + aa[1]);
}
