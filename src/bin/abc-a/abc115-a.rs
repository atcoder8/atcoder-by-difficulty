use proconio::input;

fn main() {
    input! {
        d: usize,
    }

    print!("Christmas");
    let eve_num = 25 - d;
    for _ in 0..eve_num {
        print!(" Eve");
    }
    println!();
}
