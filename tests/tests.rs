// running with command line output
// cargo test -- --no-capture

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    //use float_cmp::approx_eq;
    use optimal_solver_15puzzle::{puzzle::Puzzle, walkingDist::WD};

    const N : usize = 4;
    
    #[test]
    /// Tests Linear Conflict
    fn test_lc() {
        let mut p = Puzzle::new(N);
        p.set_state(& "5 7 3 12 15 13 14 8 0 10 9 6 1 4 2 11", N);
        println!("{}", p);

        println!("{}", p.manhattan_dist(N));
        
        
        assert_eq!(p.linear_conflict(), 4);
    }

    #[test]
    /// Tests Manhattan Distance 
    fn test_md() {
        
    }


    #[test]
    /// Tests Walking Distance
    fn test_wd() {
        let mut p = Puzzle::new(N);
        p.set_state(& "5 7 3 12 15 13 14 8 0 10 9 6 1 4 2 11", N);
        let mut wd : WD= Default::default();

        wd.gen(N);        
    }

   
}
