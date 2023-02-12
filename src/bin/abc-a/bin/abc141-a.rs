use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = if s == "Sunny" {
        "Cloudy"
    } else if s == "Cloudy" {
        "Rainy"
    } else {
        "Sunny"
    };
    println!("{}", ans);
}
