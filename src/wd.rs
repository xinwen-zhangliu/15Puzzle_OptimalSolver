use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct WD {
    //open: Vec<Vec<Vec<u8>>>,
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
    pub(crate) fn new() -> Self {
        WD {
            patterns: vec![0u64; 24964],
            moves: vec![0u8; 24964],
            link: vec![vec![vec![0u16; 4]; 2]; 24964],
            top: 0,
            end: 0,
        }
    }

    /// Creates the walking distance database
    pub fn gen(&mut self, n: usize) {
    
        let mut table: u64 = 0;

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
                // if counter >= 64 {
                //     i_table += 1;
                //     counter = 0;
                // }
                counter += 3;
                //table[i_table] = (table[i_table] << 3) | TABLE[i][j] as u64;
                table = (table << 3) | TABLE[i][j] as u64;
            }
        }

        let max = i_table;
        i_table = 0;

        // inserting the starting state with depth 0
        self.patterns[0] = table;
        self.moves[0] = 0;
        for i in 0..2 {
            for j in 0..4 {
                self.link[0][i][j] = 24964;
            }
        }

        let mask = (1 << 3) - 1;
        let mut end = 0;
        self.top = 0;
        self.end = 1;
        while self.top < self.end {
            table = self.patterns[self.top];
            depth = self.moves[self.top];
            depth += 1;
            self.top += 1;
            println!("{:b}", table);
            for i in (0..n).rev() {
                piece = 0; // reset the piece
                for j in (0..n).rev() {
                    if i_table > 0 {}
                    TABLE[i][j] = (table & mask) as u8;
                    table >>= 3;
                    piece += TABLE[i][j] as usize;

                    if table == 0 {
                        i_table += 1;
                    }
                }
                if piece == 3 {
                    space = i as i8;
                }
            }

            println!("{:?}", TABLE);
            //break;
            println!("{}", space);

            // move piece up
            if space + 1 < 4 as i8 {
                println!("move piece up");
                piece = (space + 1) as usize;
                for i in 0..n {
                    if TABLE[piece][i] > 0 {
                        TABLE[piece][i] -= 1;
                        TABLE[space as usize][i] += 1;
                        self.write_link(depth, 0, i, &mut TABLE);
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
                        self.write_link(depth, 1, i, &mut TABLE);
                        TABLE[piece][i] += 1;
                        TABLE[space as usize][i] -= 1;
                    }
                }
            }
        }
    }


    /// Returns the binary representation of a table
    pub fn get_bin(TABLE: &mut Vec<Vec<u8>>) -> u64 {
        let mut table: u64 = 0;
        for i in 0..4 {
            for j in 0..4 {
                table = (table << 3) | TABLE[i][j] as u64;
            }
        }
        table
    }

    /// Links a state to all the possible next ones
    pub fn write_link(
        &mut self,
        //n: usize,
        moves: u8,
        vect: usize,
        piece: usize,
        TABLE: &mut Vec<Vec<u8>>,
    ) {
        println!("writelink");
        let mut table: u64 = 0;
        for i in 0..4 {
            for j in 0..4 {
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
        //        counter -= 1;
        println!("{}, {}", counter, self.end);
        if counter == self.end {
            println!("{:b}", table);
            self.patterns[self.end] = table;
            self.moves[self.end] = moves;
            self.end += 1;
            for j in 0..2 {
                for k in 0..4 {
                    self.link[counter][j][k] = 24964;
                }
            }
        }

        let j = self.top - 1;
        self.link[j][vect][piece] = counter as u16;
        self.link[counter][vect ^ 1][piece] = j as u16;
    }

    /// Returns the number of moves for a certain pattern
    pub fn get_moves(&self, pattern: u64) -> u8 {
        let mut moves: u8 = 80;
        for i in 0..self.patterns.len() {
            if self.patterns[i] == pattern {
                moves = self.moves[i];
            }
        }

        moves
    }
}

impl Default for WD {
    fn default() -> Self {
        Self::new()
    }
}
