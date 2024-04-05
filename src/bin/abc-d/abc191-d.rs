use num_integer::{div_ceil, div_floor, Roots};
use proconio::input;
use regex::Regex;

const SCALER: i64 = 10000;

fn main() {
    input! {
        (x, y, r): (String, String, String),
    }

    let re = Regex::new(r"^(?<sign>-?)(?<integer>[0-9]+)(?<decimal>\.[0-9]+)?").unwrap();

    let parse = |s: &str| {
        let capture = re.captures(s).unwrap();

        let sign = if capture.name("sign").unwrap().as_str().is_empty() {
            1
        } else {
            -1
        };

        let integer = capture
            .name("integer")
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap()
            * SCALER;

        let decimal = match capture.name("decimal") {
            Some(dec) => {
                let mut dec = dec.as_str()[1..].to_owned();
                dec += &"0".repeat(4 - dec.len());

                dec.parse::<i64>().unwrap()
            }
            None => 0,
        };

        sign * (integer + decimal)
    };

    let (x, y, r) = (parse(&x), parse(&y), parse(&r));

    let mut ans = 0;
    let min_x = div_ceil(x - r, SCALER) * SCALER;
    let max_x = div_floor(x + r, SCALER) * SCALER;
    for x0 in (min_x..=max_x).step_by(SCALER as usize) {
        let sq_dy = r.pow(2) - (x - x0).pow(2);
        let dy = sq_dy.sqrt();

        let min_y = div_ceil(y - dy, SCALER) * SCALER;
        let max_y = div_floor(y + dy, SCALER) * SCALER;
        ans += (max_y - min_y) / SCALER + 1;
    }

    println!("{}", ans);
}
