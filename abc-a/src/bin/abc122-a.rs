use proconio::input;

fn main() {
    input! {
        b: char,
    }

    let ans = match b {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => panic!(),
    };
    println!("{}", ans);
}
