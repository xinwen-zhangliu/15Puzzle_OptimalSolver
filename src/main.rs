use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::puzzle::Puzzle;
use rand_distr::{Distribution, Poisson};

#[allow(non_snake_case)]
pub mod puzzle;

fn main() {
    println!("Hello, world!");
    let state = "14 13 15 7 11 12 9 5 6 0 2 1 4 8 10 3";
    //let state = "1 4 2 3 13 6 7 8 5 10 11 0 9 14 15 12"; //example MD =9
    //let state = "13 5 4 10 9 12 8 14 2 3 7 1 0 15 11 6";
    //t state =     "14 7 8 2 13 11 10 4 9 12 5 0 3 6 1 15";
    //let state = "13 11 4 12 1 8 9 15 6 5 14 2 7 3 10 0";

    //let state = "11 4 0 8 6 10 5 13 12 7 14 3 1 2 9 15";
    let mut puzzle = Puzzle::new();
    puzzle.set_state(state, 4);

    puzzle.manhattan_dist(4);

    // let poi = Poisson::new(1.0).unwrap();
    // let v = poi.sample(&mut rand::thread_rng());
    // println!("{} is from a Poisson(2) distribution", v);

    // let vec = vec![1, 2, 3];
    // let tmp = vec![1, 2, 3];
    // let mut hash = DefaultHasher::new();
    // vec.hash(&mut hash);
    // println!("{}", hash.finish());
    // let mut hash = DefaultHasher::new();
    // tmp.hash(&mut hash);
    // println!("{}", hash.finish());
}
