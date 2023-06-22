use std::fs;

use optimal_solver_15puzzle::puzzle::Puzzle;
use optimal_solver_15puzzle::search::Search;
//use rand_distr::{Distribution, Poisson};

const N: usize = 4;
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

    //  let state = "";

    //  let mut puzzle = Puzzle::new(N);
    //  //let mut p = Puzzle::new(N);
    //  // p.set_state(original.clone(), N);
    //  puzzle.set_state(state, N);
    //  println!("{}", puzzle);

    //  let mut  moves =  Search::AStar(&mut puzzle);

    // //  println!("{}", "LLDLUURDDRURDDLUUULDDRUR|URULDDDRRURULURDLLUL".len());
    // // let sec =  "URULDDDRRURULURDLLUL".chars().rev().collect::<String>();
    //  //puzzle.perform_moves("ULLDRUURULLDDLURDDLUURDRDLURUULDDRURDDLURUULLL".to_owned());
    //  moves.0.perform_moves(moves.1.clone());

    //  //ULLLURDLDRRURDLULUULDDD|RRUULULDDDRUULDDRURUURDDDLUUURDDDLUU

    //  println!("{}", moves.0);
    // println!("{}", moves.1.len());

    //   //println!("{}", p);

    let file_contents = fs::read_to_string("../../tests/raw.txt").expect("Unable to read file");
    let mut lines: Vec<String> = Vec::new();

    for line in file_contents.lines() {
        lines.push(line.trim().to_owned());
    }

    let mut cases: Vec<(String, u32, u32, u32, u32, u32, u32)> =
        vec![("".to_owned(), 0u32, 0u32, 0u32, 0u32, 0u32, 0u32); 100];
    for i in 0..100 {
        let mut state: String = "".to_owned();
        match lines[i * 10].split_once(' ') {
            Some((key, value)) => {
                state = value.to_owned();
                println!("{}", state);
            }
            None => {
                continue;
            }
        }

        cases[i] = (
            state.trim().to_owned(),
            lines[i * 10 + 1].trim().parse::<u32>().unwrap(),
            lines[i * 10 + 4].trim().parse::<u32>().unwrap(),
            lines[i * 10 + 6].trim().parse::<u32>().unwrap(),
            lines[i * 10 + 7].trim().parse::<u32>().unwrap(),
            lines[i * 10 + 8].trim().parse::<u32>().unwrap(),
            lines[i * 10 + 9].trim().parse::<u32>().unwrap(),
        );
        println!("{:?} {}", cases[i], i);
    }

    let mut solutions: Vec<(Puzzle, String, usize)> = Vec::new();
    let mut sum = 0;
    //    let i = 15;

    // 16 33 53=52
    for i in 53..100 {
        let mut puzzle = Puzzle::new(N);
        puzzle.set_state(&cases[i].0, N);
        println!("{:?}", cases[i]);
        println!("{}", puzzle);
        let mut moves = Search::AStar(&mut puzzle);
        moves.0.perform_moves(moves.1.clone());
        println!("{}", moves.0);
        println!("{}", moves.1.len());
        solutions.push((moves.0, moves.1.clone(), moves.1.len()));
        sum += moves.1.len();
    }

    println!("Average : {}", sum as f64 / 100.0);

    // for i in 0..100 {
    //     println!("{} {} {}", solutions[i].0, solutions[i].1, solutions[i].2);
    // }



    
}
