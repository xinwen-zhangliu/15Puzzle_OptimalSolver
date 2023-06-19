use crate::{puzzle::Puzzle, wd::WD};

pub struct Search{
    
}
const N : usize = 4;

impl Search {

    
    /// Goal state
    /// 0  1  ... (n*n)-1
    pub fn AStar(start : &mut Puzzle  ){

        let mut wd =  WD::new();
        wd.gen(N);
        
        let mut openL : Vec<Puzzle>= Vec::new();
        let mut closedL : Vec<Puzzle> = Vec::new();

        let mut iterator = 0;

        if iterator == 0 {
            // set depth cost os start state to zero
            start.set_depth(0);
            // calculate HH value from start state to goal state Eq3
            
            // calculate


            while !openL.is_empty(){
                // get the state with lowest f(s) from openL
                let current = openL[0];
                
            }
        }
        
    }
}
