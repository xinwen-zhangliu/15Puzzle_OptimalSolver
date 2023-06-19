use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::{puzzle::Puzzle, walkingDist::WD};

pub struct Search {}
const N: usize = 4;
#[allow(non_snake_case)]
impl Search {
    /// Goal state
    /// 0  1  ... (n*n)-1
    pub fn AStar(start: &mut Puzzle) {
        let mut wd = WD::new();
        wd.gen(N);

        let mut minheapF: BinaryHeap<Reverse<Puzzle>> = BinaryHeap::new();
        let mut hashmapF: HashMap<u64, String> = HashMap::new();

        let mut minheapB: BinaryHeap<Reverse<Puzzle>> = BinaryHeap::new();
        let mut hashmapB: HashMap<u64, String> = HashMap::new();
        // let mut openL: Vec<Puzzle> = Vec::new();
        // let mut closedL: Vec<Puzzle> = Vec::new();

        let mut iteratorF: u32 = 0;
        let mut iteratorB: u32 = 0;

        if iteratorF == 0 {
            // set depth cost os start state to zero
            start.set_depth(0);
            // calculate HH value from start state to goal state  + depth cost
            let eval = 0 + start.get_eval(&mut wd);
            start.set_eval(eval);
            minheapF.push(Reverse(start.clone()));
            //            closedL.push(start.clone());
            hashmapF.insert(start.hasher(), start.get_path());
        }
        let state = "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15";
        let mut original = Puzzle::new(N);
        original.set_state(state, N);
        if iteratorB == 0 {
            // set depth cost os start state to zero
            original.set_depth(0);
            // calculate HH value from start state to goal state  + depth cost
            let eval = 0 + original.get_eval(&mut wd);
            original.set_eval(eval);

            minheapB.push(Reverse(original.clone()));

            hashmapB.insert(original.hasher(), original.get_path());
        }

        loop {
            let str = Self::expand(
                &mut minheapF,
                &mut hashmapF,
                &mut hashmapB,
                &mut iteratorF,
                &mut wd,
            );
            if str != "" {
                println!("{}", str);
                break;
            }
            
            let str = Self::expand(
                &mut minheapB,
                &mut hashmapB,
                &mut hashmapF,
                &mut iteratorB,
                &mut wd,
            );
            if str != "" {
                println!("{}", str);
                break;
            }


            println!("{} , {} ", iteratorF, iteratorB);
        }
    }

    pub fn expand(
        minheapF: &mut BinaryHeap<Reverse<Puzzle>>,
        hashmapF: &mut HashMap<u64, String>,
        hashmapB: &mut HashMap<u64, String>,
        iterator: &mut u32,
        wd: &mut WD,
    ) -> String {
        let mut path = "".to_owned();

        while !minheapF.is_empty() {
            // get the state with lowest f(s) from openL

            let current = minheapF.pop().unwrap();
            if current.0.manhattan_dist(N) == 0 {
                //println!("sol = {}", current.0.get_path());
                return current.0.get_path();
                //                    break;
            }

            for i in 1..5 {
                let mut copy = current.0.clone();
                if copy.move_tile(i, N) {
                    // check if copy is not in closed, then we add it to
                    if !hashmapF.contains_key(&copy.hasher()) {
                        let last = copy.get_depth();
                        copy.set_depth(last + 1);

                        let eval = last + 1 + copy.get_eval(wd);
                        copy.set_eval(eval);

                        hashmapF.insert(copy.hasher(), copy.get_path());
                        minheapF.push(Reverse(copy.clone()));

                        if hashmapB.contains_key(&copy.hasher()) {
                            path = copy.get_path();
                            path.push_str("|");
                            path.push_str(
                                &hashmapB
                                    .get(&copy.hasher())
                                    .unwrap()
                                    .chars()
                                    .rev()
                                    .collect::<String>(),
                            );
                            return path;
                        }

                        *iterator = *iterator + 1;
                    }
                }
            }

            if *iterator % 15000 == 0 {
                return path;
            }
        }
        path
    }
}
