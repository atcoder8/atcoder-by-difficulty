use im_rc::HashSet;
use proconio::input;

const GRID_LEN: usize = 100;

fn main() {
    input! {
        n: usize,
        corner_pairs: [((usize, usize, usize), (usize, usize, usize)); n],
    }

    let mut grid = vec![vec![vec![n; GRID_LEN]; GRID_LEN]; GRID_LEN + 1];
    for (rect_idx, &((x1, y1, z1), (x2, y2, z2))) in corner_pairs.iter().enumerate() {
        for x in x1..x2 {
            for y in y1..y2 {
                for z in z1..z2 {
                    grid[x][y][z] = rect_idx;
                }
            }
        }
    }

    for (rect_idx, &((x1, y1, z1), (x2, y2, z2))) in corner_pairs.iter().enumerate() {
        let mut set = HashSet::new();

        for x in x1..x2 {
            for y in y1..y2 {
                for z in z1..z2 {
                    if x > 0 {
                        set.insert(grid[x - 1][y][z]);
                    }
                    if x < GRID_LEN - 1 {
                        set.insert(grid[x + 1][y][z]);
                    }
                    if y > 0 {
                        set.insert(grid[x][y - 1][z]);
                    }
                    if y < GRID_LEN - 1 {
                        set.insert(grid[x][y + 1][z]);
                    }
                    if z > 0 {
                        set.insert(grid[x][y][z - 1]);
                    }
                    if z < GRID_LEN - 1 {
                        set.insert(grid[x][y][z + 1]);
                    }
                }
            }
        }

        set.remove(&n);
        set.remove(&rect_idx);

        println!("{}", set.len());
    }
}
