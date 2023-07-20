use proconio::input;

fn main() {
    input! {
        m: usize,
    }

    let ans = if m < 100 {
        "00".to_owned()
    } else if m <= 5000 {
        format!("{:02}", m / 100)
    } else if m <= 30000 {
        (m / 1000 + 50).to_string()
    } else if m <= 70000 {
        ((m / 1000 - 30) / 5 + 80).to_string()
    } else {
        "89".to_owned()
    };
    println!("{}", ans);
}
