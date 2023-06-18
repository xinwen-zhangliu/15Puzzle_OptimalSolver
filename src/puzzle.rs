use std::fmt;

use rand::{rngs::StdRng, SeedableRng};
use rand_distr::{Distribution, Uniform};

/// Respresents an instance of the 15 puzzle
pub struct Puzzle {
    state: Vec<Vec<u8>>,
    path: String,
    scramble: String,
    space: (usize, usize),
    n: usize,
    depth: u8,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        //let mut str = "";
        for i in 0..self.state.len() {
            for j in 0..self.state[i].len() {
                let str: &str = &self.state[i][j].to_string();
                fmt.write_str(str)?;
                if self.state[i][j] < 10 {
                    fmt.write_str("  ")?;
                } else {
                    fmt.write_str(" ")?;
                }
            }
        }
        Ok(())
    }
}

impl Puzzle {
    pub fn new(n: usize) -> Self {
        Puzzle {
            state: vec![vec![0u8; n]; n],
            path: "".to_owned(),
            scramble: "".to_owned(),
            space: (3, 3),
            n,
            depth: 0,
        }
    }

    /// Initializes the puzzle in solved state
    fn initialize_puzzle(&mut self) {
        for i in 0..self.state[0].len() {
            self.state[0][i] = (i * 4 + 1) as u8;
            self.state[1][i + 1] = (i * 4 + 2) as u8;
            self.state[2][i + 2] = (i * 4 + 3) as u8;
            self.state[3][i + 3] = (i * 4 + 4) as u8;
        }
    }

    /// The state gets set according to an ordered string of numbers,
    /// representing the tiles.
    pub fn set_state(&mut self, state: &str, n: usize) {
        let mut counter: usize = 0;
        let numbers: Vec<u8> = state.split(" ").map(|x| x.parse::<u8>().unwrap()).collect();
        for i in 0..n {
            for j in 0..n {
                self.state[i][j] = numbers[counter];
                counter += 1;
            }
        }
    }

    ///
    pub fn get_state(&self) -> &str {
        let state: &str = "";
        // for i in 0..self.state[0].len() {
        //     self.state[i]
        //     for j in 0..self.state[0].len() {

        //     }
        // }

        state
    }

    /// Determines the Manhattan distance of a state
    pub fn manhattan_dist(&self, n: usize) -> u32 {
        dbg!(&self.state);

        let mut sum: u32 = 0;
        for i in 0..n {
            for j in 0..n {
                let num = self.state[i][j];
                if num == 0 {
                    continue;
                }
                let x: u8 = num % n as u8;
                let y: u8 = (num) / n as u8;
                // println!("{}: {},{} ", num, y, x);

                if y > i as u8 {
                    sum += (y - i as u8) as u32;
                } else {
                    sum += (i as u8 - y) as u32;
                }

                if x > j as u8 {
                    sum += (x - j as u8) as u32;
                } else {
                    sum += (j as u8 - x) as u32;
                }

                // 0  1  2  3
                // 4  5  6  7
                // 8  9  10 11
                // 12 13 14 15
            }
        }

        sum
    }

    /// Finds and adds the total cost of linear conflicts for a state
    /// Two tiles t_i and t_j are in linear conflict if both tiles are positioned
    /// in their goal row or column but in the wrong order, i.e. they are reversed
    /// relative to their goal location.
    pub fn linear_conflict(&self) -> u32 {
        let n = self.state[0].len();
        let mut last_number;
        //let mut index = 0;

        let mut lc = 0;

        // check rows
        for i in 0..n {
            last_number = self.state[i][0];
            for j in 0..n {
                if self.state[i][j] < last_number {
                    //bigger = self.state[i][j];
                    if self.state[i][j] / n as u8 == last_number / n as u8
                        && last_number / n as u8 == i as u8
                    {
                        lc += 2;
                    }
                }
                last_number = self.state[i][j];
            }
        }

        // check columns
        for j in 0..n {
            last_number = self.state[0][j];
            for i in 0..n {
                if self.state[i][j] < last_number {
                    if self.state[i][j] % n as u8 == last_number % n as u8
                        && last_number % n as u8 == j as u8
                    {
                        lc += 2;
                    }
                }
                last_number = self.state[i][j];
            }
        }
        lc
    }

    /// Scrambles the puzzle with n random moves
    pub fn scramble(&mut self, n: usize, size: usize) {
        let mut r = StdRng::seed_from_u64(42);
        let moves = Uniform::from(1..4);
        let mut previous = moves.sample(&mut r);
        let mut next = moves.sample(&mut r);

        for i in 0..n {
            while !self.move_tile(next, size) {
                while previous == next {
                    next = moves.sample(&mut r);
                }
            }
            next = moves.sample(&mut r);
            previous = next;
        }
    }

    /// Returns the horizontal walking distance representation of a state
    fn wd_h(&mut self, n: usize) -> u64 {
        let mut rep: u64 = 0;
        let mut table = vec![vec![0u8; n]; n];
        for i in 0..n {
            for j in 0..n {
                let row = self.state[i][j] / n as u8;
                table[i][row as usize] += 1;
            }
        }

        for i in (0..n).rev() {
            for j in (0..n).rev() {
                rep = (rep << 3) | table[i][j] as u64;
            }
        }

        rep
    }

    /// Returns the vertical walking distance representation of a state
    fn wd_v(&mut self, n: usize) -> u64 {
        let mut rep: u64 = 0;
        let mut table = vec![vec![0u8; n]; n];
        for j in 0..n {
            for i in 0..n {
                let column = self.state[i][j] % n as u8;
                table[j][column as usize] += 1;
            }
        }

        for i in (0..n).rev() {
            for j in (0..n).rev() {
                rep = (rep << 3) | table[i][j] as u64;
            }
        }
        rep
    }

    pub fn set_depth(&mut self, depth: u8) {
        self.depth = depth;
    }

    pub fn set_space(&mut self, i: usize, j: usize) {
        self.space.0 = i;
        self.space.1 = j;
    }

    /// Slides available tiles in the required direction
    /// m = 1 : move up
    /// m = 2 : move right
    /// m = 3 : move down
    /// m = 4 : move left
    pub fn move_tile(&mut self, m: usize, n: usize) -> bool {
        let slide = |t1: (usize, usize), t2: (usize, usize), table: &mut Vec<Vec<u8>>| {
            let tile = table[t1.0][t1.1];
            table[t1.0][t1.1] = table[t2.0][t2.1];
            table[t2.0][t2.1] = tile;
        };

        match m {
            1 => {
                if self.space.0 == n - 1 {
                    return false;
                } else {
                    slide(
                        (self.space.0, self.space.1),
                        (self.space.0 + 1, self.space.1),
                        &mut self.state,
                    );
                    self.set_space(self.space.0 + 1, self.space.1);
                    return true;
                }
            }

            2 => {
                if self.space.1 == 0 {
                    return false;
                } else {
                    slide(
                        (self.space.0, self.space.1),
                        (self.space.0, self.space.1 - 1),
                        &mut self.state,
                    );
                    self.set_space(self.space.0, self.space.1 - 1);
                    return true;
                }
            }

            3 => {
                if self.space.0 == 0 {
                    return false;
                } else {
                    slide(
                        (self.space.0, self.space.1),
                        (self.space.0 - 1, self.space.1),
                        &mut self.state,
                    );
                    self.set_space(self.space.0 - 1, self.space.1);
                    return true;
                }
            }

            4 => {
                if self.space.1 == n - 1 {
                    return false;
                } else {
                    slide(
                        (self.space.0, self.space.1),
                        (self.space.0, self.space.1 + 1),
                        &mut self.state,
                    );
                    self.set_space(self.space.0, self.space.1 + 1);
                    return true;
                }
            }

            _ => return false,
        }
    }
}
