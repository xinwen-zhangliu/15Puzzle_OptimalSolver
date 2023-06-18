use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};


pub struct WD {
    open: Vec<Vec<Vec<u8>>>,
    patterns: Vec<u64>,
    moves: Vec<u8>,
    link: Vec<Vec<Vec<u16>>>,
    top: usize,
    end: usize,
    //table : Vec<Vec<u8>>
}

#[derive(PartialEq, Eq, Hash)]
pub struct Table {
    n_moves: u8,
    table: [u64; 3],
}
impl Table {
    pub fn hasher(&mut self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl WD {
    pub fn create_db(&mut self, n: usize) {
        let mut table = vec![0u64; 3];

        //let open: Vec<Vec<usize>> = Vec::new();

        let mut space: i8 = 0;
        let mut piece: usize;
        let mut depth: u8 = 0;
        let mut TABLE: Vec<Vec<u8>> = vec![vec![0u8; n]; n]; // initializing in zeros

        // Fill up the starting state of the board
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    TABLE[i][j] = 4;
                }
            }
        }
        TABLE[0][0] = 3; // space in first row

        // Creating puzzle binary representation
        let mut i_table: usize = 0;
        let mut counter = 0;
        for i in 0..n {
            for j in 0..n {
                if counter >= 64 {
                    i_table += 1;
                    counter = 0;
                }
                counter += 3;
                table[i_table] = (table[i_table] << 3) | TABLE[i][j] as u64;
            }
        }

        let max = i_table;
        i_table = 0;

        // inserting the starting state with depth 0
        self.patterns[0] = table[i_table];
        self.moves[0] = 0;
        for i in 0..n {
            for j in 0..n {
                self.link[0][i][j] = 24964;
            }
        }

        let mask = (1 << 3) - 1;
        let mut end = 0;
        self.top = 0;
        self.end = 1;
        while self.top < self.end {
            depth += 1;
            end += 1;

            for i in (0..n).rev() {
                piece = 0; // reset the piece
                for j in (0..n).rev() {
                    if i_table > 0 {}
                    TABLE[i][j] = (table[i_table] & mask) as u8;
                    table[i_table] >>= 3;
                    piece += TABLE[i][j] as usize;
                    if table[i_table] == 0 {
                        i_table += 1;
                    }

                    if piece == 3 {
                        space = 1;
                    }
                }
            }

            // move piece up
            if space + 1 < n as i8 {
                piece = (space + 1) as usize;
                for i in 0..n {
                    if TABLE[piece][i] > 0 {
                        TABLE[piece][i] -= 1;
                        TABLE[space as usize][i] += 1;
                        self.write_link(n, depth, 0, i, &mut TABLE);
                        TABLE[piece][i] += 1;
                        TABLE[space as usize][i] -= 1;
                    }
                }
            }

            // move piece down

            if space - 1 >= 0 {
                piece = (space - 1) as usize;
                for i in 0..n {
                    if TABLE[piece][i] > 0 {
                        TABLE[piece][i] -= 1;
                        TABLE[space as usize][i] += 1;
                        self.write_link(n, depth, 1, i, &mut TABLE);
                        TABLE[piece][i] += 1;
                        TABLE[space as usize][i] -= 1;
                    }
                }
            }
        }
    }

    /// Links a state to all the possible next ones
    pub fn write_link(
        &mut self,
        n: usize,
        moves: u8,
        vect: usize,
        piece: usize,
        TABLE: &mut Vec<Vec<u8>>,
    ) {
        let mut table: u64 = 0;
        for i in 0..n {
            for j in 0..n {
                table = (table << 3) | TABLE[i][j] as u64;
            }
        }

        let mut counter: usize = 0;
        while counter < self.end {
            if self.patterns[counter] == table {
                break;
            }
            counter += 1;
        }

        if counter == self.end {
            self.patterns[self.end] = table;
            self.moves[self.end] = moves;
            self.end += 1;
            for j in 0..n {
                for k in 0..n {
                    self.link[counter][j][k] = 24964;
                }
            }
        }

        let j = self.top - 1;
        self.link[j][vect][piece] = counter as u16;
        self.link[counter][vect ^ 1][piece] = j as u16;
    }

    /// Writes the patters, numbers of required moves and linker table in file.
    pub fn write_to_file(&mut self) {}
}
