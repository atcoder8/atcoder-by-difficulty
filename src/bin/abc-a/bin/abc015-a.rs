use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }

    if a.chars().count() > b.chars().count() {
        println!("{}", a);
    } else {
        println!("{}", b);
    }
}
