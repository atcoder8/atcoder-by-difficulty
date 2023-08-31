use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut ans = 0;
    let mut ord: Vec<Option<usize>> = vec![None; n];
    let mut low: Vec<Option<usize>> = vec![None; n];
    let mut stack: Vec<(Option<usize>, usize, bool)> = vec![(None, 0, false)];
    while let Some((par, cur, back)) = stack.pop() {
        if back {
            if let Some(par) = par {
                low[par] = low[par].min(low[cur]);

                if ord[par] < low[cur] {
                    ans += 1;
                }
            }

            continue;
        }

        if ord[cur].is_some() {
            if let Some(par) = par {
                low[par] = low[par].min(ord[cur]);
            }

            continue;
        }

        let t = match par {
            Some(par) => ord[par].unwrap() + 1,
            None => 0,
        };

        ord[cur] = Some(t);
        low[cur] = Some(t);

        stack.push((par, cur, true));

        for &next in &graph[cur] {
            if Some(next) != par {
                stack.push((Some(cur), next, false));
            }
        }
    }

    println!("{}", ans);
}
