use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        lines: [String; n],
    }

    let rainfall_sections = lines
        .iter()
        .map(|line| parse_rainfall_section(line))
        .sorted_unstable_by_key(|v| v.1);

    let mut integrated_sections: Vec<(usize, usize)> = vec![];
    for (mut start, finish) in rainfall_sections {
        while let Some(&(last_start, last_finish)) = integrated_sections.last() {
            if last_finish < start {
                break;
            }

            integrated_sections.pop();

            start = start.min(last_start);
        }

        integrated_sections.push((start, finish));
    }

    let ans = integrated_sections
        .iter()
        .map(|&(start, finish)| format!("{:04}-{:04}", start, finish))
        .join("\n");
    println!("{}", ans);
}

fn parse_rainfall_section(line: &str) -> (usize, usize) {
    let mut iter = line.trim().split("-");
    let start = iter.next().unwrap().parse::<usize>().unwrap();
    let finish = iter.next().unwrap().parse::<usize>().unwrap();

    let (h1, mut m1) = (start / 100, start % 100);
    m1 = m1 / 5 * 5;

    let (mut h2, mut m2) = (finish / 100, finish % 100);
    m2 = (m2 + 4) / 5 * 5;
    if m2 == 60 {
        h2 += 1;
        m2 = 0;
    }

    (h1 * 100 + m1, h2 * 100 + m2)
}
