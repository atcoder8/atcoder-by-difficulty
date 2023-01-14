use proconio::input;

fn main() {
    input! {
        mut ll: [usize; 3],
    }

    ll.sort_unstable();

    let ans = if ll[0] == ll[1] { ll[2] } else { ll[0] };
    println!("{}", ans);
}
