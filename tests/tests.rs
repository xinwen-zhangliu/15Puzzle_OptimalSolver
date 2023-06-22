// running with command line output
// cargo test -- --no-capture

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use std::{cmp::Reverse, collections::BinaryHeap, fs, io};

    //use float_cmp::approx_eq;
    use optimal_solver_15puzzle::{puzzle::Puzzle, search::Search, walkingDist::WD};

    const N: usize = 4;

    #[test]
    /// Tests Linear Conflict
    fn test_lc() {
        let mut p = Puzzle::new(N);
        p.set_state(&"5 7 3 12 15 13 14 8 0 10 9 6 1 4 2 11", N);
        println!("{}", p);

        println!("{}", p.manhattan_dist(N));

        assert_eq!(p.linear_conflict(), 4);
    }

    #[test]
    /// Tests Manhattan Distance
    fn test_md() {
        let file_contents = fs::read_to_string("tests/raw.txt").expect("Unable to read file");
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
        for i in 0..100 {
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


        for i in 0..100{

            println!("{} {} {}", solutions[i].0, solutions[i].1, solutions[i].2);
        }
    }
    #[test]
    fn minheap() {
        let mut minheap: BinaryHeap<Reverse<Puzzle>> = BinaryHeap::new();

        for i in 0..10 {
            let mut p = Puzzle::new(N);
            p.set_eval(i);
            p.set_depth(i);
            minheap.push(Reverse(p));
        }

        assert_eq!(minheap.pop().unwrap().0.get_depth(), 0)
    }

    #[test]
    /// Tests Walking Distance
    fn test_wd() {
        let mut p = Puzzle::new(N);
        p.set_state(&"5 7 3 12 15 13 14 8 0 10 9 6 1 4 2 11", N);
        let mut wd: WD = Default::default();

        wd.gen(N);
    }
}
