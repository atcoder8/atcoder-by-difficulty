use im_rc::HashMap;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut tree = vec![(0, None)];
    let mut cur = 0;
    let mut note = HashMap::new();

    let mut tails = vec![];
    for _ in 0..q {
        input! {
            query_type: String,
        }

        match &query_type[..] {
            "ADD" => {
                input! {
                    x: usize,
                }

                tree.push((cur, Some(x)));
                cur = tree.len() - 1;
            }

            "DELETE" => cur = tree[cur].0,

            "SAVE" => {
                input! {
                    y: usize,
                }

                note.insert(y, cur);
            }

            "LOAD" => {
                input! {
                    z: usize,
                }

                cur = *note.get(&z).unwrap_or(&0);
            }

            _ => panic!(),
        }

        tails.push(tree[cur].1);
    }

    println!(
        "{}",
        tails
            .iter()
            .map(|tail| tail.map_or("-1".to_string(), |tail| tail.to_string()))
            .join(" ")
    );
}
