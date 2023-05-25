/// Respresents an instance of the 15 puzzle
pub struct Puzzle {
    state: Vec<Vec<u8>>,
    path: String,
    empty_tile: (usize, usize),
}

impl Puzzle {
    pub fn new() -> Self {
        Puzzle {
            state: vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 0],
            ],
            path: "".to_string(),
            empty_tile : (3, 3),
        }
    }

    /// Initializes the puzzle in solved state
    fn initialize_puzzle(&mut self) {
        for i in 0..4 {
            self.state[0][i] = (i * 4 + 1) as u8;
            self.state[1][i + 1] = (i * 4 + 2) as u8;
            self.state[2][i + 2] = (i * 4 + 3) as u8;
            self.state[3][i + 3] = (i * 4 + 4) as u8;
        }
    }

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

    /// Determines the Manhattan distance of a state
    ///
    pub fn manhattan_dist(&self, n: usize) -> u32 {
        dbg!(&self.state);

        let mut sum: u32 = 0;
        for i in 0..n {
            for j in 0..n {
                let num = self.state[i][j];
                if num == 0 {
                    continue;
                }
                let x : u8 = num % n as u8;
                let y : u8 = (num) / n as u8;
                // if num == 0  {
                //     x = (n-1) as u8 ;
                //     y = (n-1) as u8;
                // }else {
                //     x = num  % n as u8 ;
                //     y = ((num-1) as f64 / n as f64) as u8 ;
                // }
                // if x == 0{
                //     x = (n-1) as u8 ;
                // }else{
                //     x -= 1;
                // }
                println!("{}: {},{} ", num, y, x);

                if y > i as u8 {
                    sum += (y-i as u8) as u32;
                }else{
                    sum += (i as u8-y ) as u32;
                }

                if x > j as u8 {
                    sum += (x-j as u8) as u32;
                }else{
                    sum += (j as u8-x ) as u32;
                }

                // 0  1  2  3
                // 4  5  6  7
                // 8  9  10 11
                // 12 13 14 15

                
                
                
                
                dbg!(sum);
            }
        }

        sum
    }

    /// Finds and adds the total cost of linear conflicts for a state
    pub fn linear_conflict(&self) -> u32 {
        0
    }

    pub fn scramble(&mut self) {}

    /// Slides available tiles in the required direction
    /// m = 1 : move right
    /// m = 2 : move left
    /// m = 3 : move up
    /// M = 4 : move down
    pub fn move_tile(&mut self, m: usize, n: usize) {
        if self.empty_tile.0 == 0 || self.empty_tile.0 == n - 1 {
            // available moves
            //
        }

        if self.empty_tile.1 == 0 || self.empty_tile.1 == n - 1 {}
    }
}
