// running with command line output
// cargo test -- --no-capture

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use std::{fs, io, collections::BinaryHeap, cmp::Reverse};
    

    //use float_cmp::approx_eq;
    use optimal_solver_15puzzle::{puzzle::Puzzle, walkingDist::WD};

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

        let mut counter = 0;
        let mut md_counter = 0;
        //let lines = file_contents.lines().collect::<Result<Vec<&str>, io::Error>>().unwrap();
        //        let lines : Lines = file_contents.lines();

        let iter = file_contents.lines().into_iter();
        
        while counter < 1000 {
            if counter % 10 == 0 {
                match iter.unwrap().split_once(' ') {
                    Some((key, value)) => {
                        println!("key: {}", key);
                        println!("value: {}", value);
                        for i in 0..6 {
                            next();
                        }
                    }
                    None => {
                        println!("expected a key-value pair");
                    }
                }
            }
            
            
            counter += 1;
        }

        
        let mut cases = 
       for line in   file_contents.lines(){
            if counter % 10 == 0 {
                match line.split_once(' ') {
                    Some((key, value)) => {
                        println!("key: {}", key);
                        println!("value: {}", value);
                       
                    }
                    None => {
                        println!("expected a key-value pair");
                    }
                }
            }
            
        }
    }
    #[test]
    fn minheap() {
        let mut minheap: BinaryHeap<Reverse<Puzzle>> = BinaryHeap::new();

        for i in 0..10{
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
