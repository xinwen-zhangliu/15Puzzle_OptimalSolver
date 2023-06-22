use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashMap},
};

use crate::{puzzle::Puzzle, walkingDist::WD};

pub struct Search;
const N: usize = 4;
#[allow(non_snake_case)]
static mut START_EVAL: u8 = 0;
impl Search {
    /// Goal state
    /// 0  1  ... (n*n)-1
    pub fn AStar(start: &mut Puzzle) -> (Puzzle, String) {
        let mut wd = WD::new();
        wd.gen(N);

        let copy = start.clone();

        let mut openF: BinaryHeap<Reverse<Puzzle>> = BinaryHeap::new();
        let mut closedF: BTreeMap<u64, String> = BTreeMap::new();

        let mut openB: BinaryHeap<Reverse<Puzzle>> = BinaryHeap::new();
        let mut closedB: BTreeMap<u64, String> = BTreeMap::new();

        let mut iteratorF: u32 = 0;
        let mut iteratorB: u32 = 0;

        if iteratorF == 0 {
            // set depth cost os start state to zero
            start.set_depth(0);
            // calculate HH value from start state to goal state  + depth cost
            let eval = 0 + start.get_eval(&mut wd);
            unsafe {
                START_EVAL = eval;
            }
            println!("Eval {}", eval);
            start.set_eval(eval);
            openF.push(Reverse(start.clone()));

            closedF.insert(start.hasher(), start.get_path());
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

            openB.push(Reverse(original.clone()));

            closedB.insert(original.hasher(), original.get_path());
        }

        loop {
            let str = Self::expandF(
                &mut openF,
                &mut closedF,
                &mut closedB,
                &mut iteratorF,
                &mut wd,
            );

            if str != "" {
                println!("{}", str);
                println!("{},{}", closedF.len(), closedB.len());
                return (copy, str);
                //                break;
            }

            let str = Self::expandB(
                &mut openB,
                &mut closedB,
                &mut closedF,
                &mut iteratorB,
                &mut wd,
            );
            if str != "" {
                println!("{}", str);

                //break;
                println!("{},{}", closedF.len(), closedB.len());
                return (original, str);
            }

            println!("{} , {} ", iteratorF, iteratorB);
        }
    }

    pub fn expandF(
        openF: &mut BinaryHeap<Reverse<Puzzle>>,
        closedF: &mut BTreeMap<u64, String>,
        closedB: &mut BTreeMap<u64, String>,
        iterator: &mut u32,
        wd: &mut WD,
    ) -> String {

        dbg!("EF");
        let mut best = 0;
        let mut puzzles: Vec<Puzzle> = Vec::new();

        let mut lowest = 80;

        let mut expand_best = if *iterator >= 15000 { true} else { false };

        let mut path = "".to_owned();

        while !openF.is_empty() {
            // get the state with lowest f(s) from openL

            let current = openF.pop().unwrap();
            println!("{}", current.0);
            
            if current.0.is_goal() && closedF.len() > 1 {
                return current.0.get_path();
            }
            puzzles.clear();
            for i in 1..5 {
                let mut copy = current.0.clone();
                if copy.move_tile(i, N) {
                   // dbg!("moving tile ", *iterator);
                    // check if copy is not in closed, then we add it to
                    if !closedF.contains_key(&copy.hasher()) {
                        let last = copy.get_depth();
                        copy.set_depth(last + 1);

                        let eval = last + 1 + copy.get_eval(wd);
                        copy.set_eval(eval);
                        if !expand_best {
                            //dbg!("not inserting");
                            closedF.insert(copy.hasher(), copy.get_path());
                            openF.push(Reverse(copy.clone()));
                        }

                        let hash = copy.hasher();

                        if closedB.contains_key(&hash) {
                            println!("{} {}", &copy, hash);
                            path = copy.get_path();
                            path.push_str("|");
                            println!("{}", &closedB.get(&hash).unwrap());
                            let backwards = &closedB
                                .get(&copy.hasher())
                                .unwrap()
                                .chars()
                                .rev()
                                .collect::<String>();

                            let mut rebuilt = "".to_owned();
                            for i in backwards.chars() {
                                match i {
                                    'U' => rebuilt.push_str("D"),
                                    'D' => rebuilt.push_str("U"),
                                    'L' => rebuilt.push_str("R"),
                                    'R' => rebuilt.push_str("L"),
                                    _ => {}
                                }
                            }

                            path += &rebuilt;
                            return path;
                        }

                        *iterator = *iterator + 1;
                    }
                    puzzles.push(copy);
                }
            }

            
            for i in 0..puzzles.len() {
                let eval = puzzles[i].get_current_eval();
                if eval < lowest {
                    lowest = eval;
                    best = i;
                }
            }
           
            if expand_best {
                //dbg!("inserting", *iterator);
                println!("{}{}" ,puzzles[best], lowest);
                println!("{},{}", closedF.len(), closedB.len());
                closedF.insert(puzzles[best].hasher(), puzzles[best].get_path());
                openF.push(Reverse(puzzles[best].clone()));                
            }
            best = 0;
            lowest = 80;
            
            if *iterator % 15000 ==0   {
                return path;
            }

           
        }
        path
    }

     pub fn expandB(
        openF: &mut BinaryHeap<Reverse<Puzzle>>,
        closedF: &mut BTreeMap<u64, String>,
        closedB: &mut BTreeMap<u64, String>,
        iterator: &mut u32,
        wd: &mut WD,
     ) -> String {

         dbg!("B");
          let mut path = "".to_owned();

        while !openF.is_empty() {
            // get the state with lowest f(s) from openL

            let current = openF.pop().unwrap();
            if current.0.is_goal() && closedF.len() > 1 {
                //println!("sol = {}", current.0.get_path());
                return current.0.get_path();
                //                    break;
            }

            for i in 1..5 {
                let mut copy = current.0.clone();
                if copy.move_tile(i, N) {
                    // check if copy is not in closed, then we add it to
                    if !closedF.contains_key(&copy.hasher()) {
                        let last = copy.get_depth();
                        copy.set_depth(last + 1);

                        let eval = last + 1 + copy.get_eval(wd);
                        copy.set_eval(eval);
                        //if eval <= current.0.get_current_eval() {
                        closedF.insert(copy.hasher(), copy.get_path());
                        openF.push(Reverse(copy.clone()));
                        //}

                        let hash = copy.hasher();

                        if closedB.contains_key(&hash) {
                            println!("{} {}", &copy, hash);
                            path = copy.get_path();
                            path.push_str("|");
                            println!("{}", &closedB.get(&hash).unwrap());
                            let backwards = &closedB
                                .get(&copy.hasher())
                                .unwrap()
                                .chars()
                                .rev()
                                .collect::<String>();

                            // path.push_str(
                            //     &closedB
                            //         .get(&copy.hasher())
                            //         .unwrap()
                            //         .chars()
                            //         .rev()
                            //         .collect::<String>()
                            // );
                            let mut rebuilt = "".to_owned();
                            for i in backwards.chars() {
                                match i {
                                    'U' => rebuilt.push_str("D"),
                                    'D' => rebuilt.push_str("U"),
                                    'L' => rebuilt.push_str("R"),
                                    'R' => rebuilt.push_str("L"),
                                    _ => {}
                                }
                            }

                            path += &rebuilt;
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
