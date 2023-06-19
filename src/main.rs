
use optimal_solver_15puzzle::puzzle::Puzzle;
use optimal_solver_15puzzle::search::Search;
//use rand_distr::{Distribution, Poisson};



const N : usize = 4;
#[allow(non_snake_case)]
//pub mod puzzle;

fn main() {
   
//    let state = "14 13 15 7 11 12 9 5 6 0 2 1 4 8 10 3";
    //let state = "4 1 2 3 0 5 6 7 8 9 10 11 12 13 14 15";

    let state = "14 1 9 6 4 8 12 5 7 2 3 0 10 11 13 15";

    //let state = "1 4 2 3 13 6 7 8 5 10 11 0 9 14 15 12"; //example MD =9
    //let state = "13 5 4 10 9 12 8 14 2 3 7 1 0 15 11 6";
    //t state =     "14 7 8 2 13 11 10 4 9 12 5 0 3 6 1 15";
    //let state = "13 11 4 12 1 8 9 15 6 5 14 2 7 3 10 0";

    //let state = "11 4 0 8 6 10 5 13 12 7 14 3 1 2 9 15";
    let mut puzzle = Puzzle::new(N);
    puzzle.set_state(state, N);
    println!("{}", puzzle);
    //puzzle.initialize_puzzle();
    puzzle.scramble(3, N);
    println!("{}, {}", puzzle, puzzle.get_path());
    //let search = Search{};
    Search::AStar(&mut puzzle);


    

    // dbg!(puzzle.manhattan_dist(N));

    // // let poi = Poisson::new(1.0).unwrap();
    // // let v = poi.sample(&mut rand::thread_rng());
    // // println!("{} is from a Poisson(2) distribution", v);

    // let vec1 = vec![0, 0, 0, 3];
    // let vec2 = vec![0, 0, 0, 3];
    // // 9607916165725134378
    // // 5031943648731548709
    // // let tmp = vec![1, 2, 3];
    // //dbg!(vec1.len());
    // let mut map = HashMap::new();
    // let mut hash = DefaultHasher::new();
    // vec1.hash(&mut hash);

    // let result = hash.finish();
    // map.insert(result, vec1);
    // println!("{}", result);


    // let mut hash2 = DefaultHasher::new();
    // vec2.hash(&mut hash2);
    // println!("{}", hash.finish());



    // let a = 2;
    // let b = 3;

    // dbg!(a << b);

    // let data = "Some data!";
    // fs::write("/tmp/foo", data).expect("Unable to write file");
    // let data = fs::read("/etc/hosts").expect("Unable to read file");
    // println!("{}", data.len());

}
