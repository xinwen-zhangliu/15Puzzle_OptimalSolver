
use optimal_solver_15puzzle::puzzle::Puzzle;
use optimal_solver_15puzzle::search::Search;
//use rand_distr::{Distribution, Poisson};



const N : usize = 4;
#[allow(non_snake_case)]
//pub mod puzzle;

fn main() {
   
    //let state = "14 13 15 7 11 12 9 5 6 0 2 1 4 8 10 3";
//     -------------
// |6 |5 |1 |0 |
// -------------
// |4 |11|7 |2 |
// -------------
// |8 |9 |14|3 |
// -------------
// |12|13|15|10|
// -------------

// ULLLURDLDRRURDLULUULDDD|RRUULULDDDRUULDDRURUURDDDLUUURDDDLUU
// 13455001,12558079

// -------------
// |2 |10|3 |12|
// -------------
// |6 |4 |1 |13|
// -------------
// |14|0 |7 |15|
// -------------
// |8 |11|5 |9 |
// -------------


    
    //let state = "4 1 2 3 0 5 6 7 8 9 10 11 12 13 14 15";

    //let state = "14 1 9 6 4 8 12 5 7 2 3 0 10 11 13 15";

    //let original = "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15";
    // let state = "4 5 7 2 9 14 12 13 0 3 6 11 8 1 15 10";
    //let state = "1 4 2 3 13 6 7 8 5 10 11 0 9 14 15 12"; //example MD =9
    
    //let state = "13 5 4 10 9 12 8 14 2 3 7 1 0 15 11 6";
    
    //t state =     "14 7 8 2 13 11 10 4 9 12 5 0 3 6 1 15";
    //let state = "13 11 4 12 1 8 9 15 6 5 14 2 7 3 10 0";
    //let state = "3 14 9 11 5 4 8 2 13 12 6 7 10 1 15 0";
    //let state = "11 4 0 8 6 10 5 13 12 7 14 3 1 2 9 15";

    let state = "";
    
    let mut puzzle = Puzzle::new(N);
    //let mut p = Puzzle::new(N);
    // p.set_state(original.clone(), N);
     puzzle.set_state(state, N);
    println!("{}", puzzle);


  
    
    
    
    let mut  moves =  Search::AStar(&mut puzzle);

    
   //  println!("{}", "LLDLUURDDRURDDLUUULDDRUR|URULDDDRRURULURDLLUL".len());
   // let sec =  "URULDDDRRURULURDLLUL".chars().rev().collect::<String>();
    //puzzle.perform_moves("ULLDRUURULLDDLURDDLUURDRDLURUULDDRURDDLURUULLL".to_owned());
    moves.0.perform_moves(moves.1.clone());


    //ULLLURDLDRRURDLULUULDDD|RRUULULDDDRUULDDRURUURDDDLUUURDDDLUU

    println!("{}", moves.0);
   println!("{}", moves.1.len());
    
     //println!("{}", p);

    // 1
    //     010
    //     001
    //     000
    //     010
    //     001
    //     000
    //     001
    //     001
    //     001
    //     010
    //     000
    //     000
    //     000
    //     001
    //     011


}
