use std::sync::{Arc, Mutex};

use rayon::prelude::*;

mod utils;
use utils::{Args, parse_args};

mod grid;
#[allow(unused_imports)]
use grid::{Grid, print_grids};

mod dict;
use dict::Dict;

struct Solver {
    side_len: usize,
    dictionary: Dict,
}

impl Solver {
    fn new(side_len: usize, dictionary: Dict) -> Self {
        Solver { side_len, dictionary }
    }

    fn does_prefix_exist(&self, prefix: &str) -> bool {
        let prefix_bytes = prefix.as_bytes();
        // TODO: is there a way to avoid collecting? no need to get an entire list
        let prefixes: Vec<String> = self.dictionary.get_trie().predictive_search(prefix_bytes).collect();
        return !prefixes.is_empty();
    }
    
    fn are_cols_valid(&self, grid: &Grid, check_cols_for_words: bool) -> bool {
        for col in 0..self.side_len {
            let mut col_str = String::new();
            for row in 0..grid.len() {
                col_str.push(grid.grid[row].chars().nth(col).unwrap());
            }
            if !self.does_prefix_exist(&col_str) {
                return false;
            }
    
            // Column shouldn't contain row word
            if check_cols_for_words {
                // TODO: maybe use a hashmap for O(1) lookup
                if grid.grid.contains(&col_str) {
                    return false;
                }
            }
        }
        return true;
    }
    
    fn is_grid_valid(&self, grid: &Grid) -> bool {
        if grid.len() != self.side_len {
            return false;
        }
        if !self.are_cols_valid(grid, true) {
            return false;
        }
        return true;
    }

    fn backtrack(
        &self,
        grid: &Grid, 
        solution_grids: &Arc<Mutex<Vec<Grid>>>,
    ){    
        if self.is_grid_valid(&grid) {
            println!("{}", grid);
            let mut vec = solution_grids.lock().unwrap();
            vec.push(grid.clone());
            return;
        }
    
        self.dictionary
        .get_word_list()
        .par_iter() // Use parallel iterator
        .filter(|word| !grid.grid.contains(*word)) // Exclude used words
        .for_each(|word| {
            if !self.are_cols_valid(&grid, false) {
                return;
            }
    
            let mut grid_clone = grid.clone();
            grid_clone.grid.push(word.clone());
    
            // Recursive call
            self.backtrack(&grid_clone, solution_grids);
    
            grid_clone.grid.pop();
        });
    }

    fn solve(&self) -> Vec<Grid>{
        // TODO: can I avoid repeated transpose solutions?
        // TODO: maybe just use a fixed-size array instead of Vec --> No bounds checking
    
        let mut initial_grid = Grid::new();
        let mut solution_grids: Arc<Mutex<Vec<Grid>>> = Arc::new(Mutex::new(Vec::new()));
    
        self.backtrack(&mut initial_grid, &mut solution_grids);
        return Arc::try_unwrap(solution_grids)
            .expect("Failed to unwrap Arc")
            .into_inner()
            .expect("   Failed to unlock Mutex");
    }
}


fn main() {
    let args: Args = parse_args();

    let solver: Solver = Solver::new(
        args.side_len, 
        Dict::new(args.dict_path.clone())
    );
    println!("Dictionary loaded with {} words.", solver.dictionary.get_word_list().len());

    solver.solve();
}

