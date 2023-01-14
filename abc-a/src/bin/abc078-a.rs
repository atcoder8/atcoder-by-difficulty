use proconio::input;

fn main() {
    input! {
        (x, y): (char, char),
    }

    let ans = match x.cmp(&y) {
        std::cmp::Ordering::Less => "<",
        std::cmp::Ordering::Equal => "=",
        std::cmp::Ordering::Greater => ">",
    };
    println!("{}", ans);
}
