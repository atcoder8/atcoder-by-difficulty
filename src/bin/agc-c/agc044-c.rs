use itertools::{enumerate, Itertools};
use proconio::{input, marker::Chars};

const THREE_POWS: [usize; 13] = [
    1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441,
];

fn main() {
    input! {
        n: usize,
        t: Chars,
    }

    let mut songs = vec![];
    for c in t {
        if c == 'S' && songs.last().is_some_and(|&song| song == 'S') {
            songs.pop();
        } else {
            songs.push(c);
        }
    }

    let mut mapping = Mapping::new(n);
    for song in songs {
        if song == 'S' {
            mapping.apply_salsa();
        } else {
            mapping.apply_rhumba();
        }
    }

    let mut ans = (0..THREE_POWS[n]).map(|x| mapping.apply(x));
    println!("{}", ans.join(" "));
}

struct Mapping {
    n: usize,
    salsa: bool,
    sigmas: Vec<Vec<usize>>,
}

impl Mapping {
    fn get_digit(x: usize, digit: usize) -> usize {
        x % THREE_POWS[digit + 1] / THREE_POWS[digit]
    }

    fn add_to_digit(mut x: usize, digit: usize, add_num: usize) -> usize {
        let d = Mapping::get_digit(x, digit);
        x -= d * THREE_POWS[digit];
        x += (d + add_num) % 3 * THREE_POWS[digit];

        x
    }

    fn new(n: usize) -> Self {
        let sigmas = (0..n).map(|digit| vec![0; THREE_POWS[digit]]).collect_vec();

        Self {
            n,
            salsa: false,
            sigmas,
        }
    }

    fn replace_by_salsa(&self, mut x: usize) -> usize {
        let mut replaced_x = 0;
        for i in 0..self.n {
            replaced_x += (3 - x % 3) % 3 * THREE_POWS[i];
            x /= 3;
        }

        replaced_x
    }

    fn apply_salsa(&mut self) {
        self.salsa = !self.salsa;
    }

    fn apply_sigma(&mut self, add_digit: usize, mut rem: usize) {
        if self.salsa {
            rem = self.replace_by_salsa(rem);
        }

        for digit in 0..add_digit {
            rem =
                Mapping::add_to_digit(rem, digit, 3 - self.sigmas[digit][rem % THREE_POWS[digit]]);
        }

        self.sigmas[add_digit][rem] = (self.sigmas[add_digit][rem] + 1 + self.salsa as usize) % 3;
    }

    fn apply_rhumba(&mut self) {
        for i in (0..self.n).rev() {
            self.apply_sigma(i, THREE_POWS[i] - 1);
        }
    }

    fn apply(&self, mut x: usize) -> usize {
        for (digit, sigma) in enumerate(&self.sigmas).rev() {
            x = Mapping::add_to_digit(x, digit, sigma[x % THREE_POWS[digit]]);
        }

        if self.salsa {
            x = self.replace_by_salsa(x);
        }

        x
    }
}
