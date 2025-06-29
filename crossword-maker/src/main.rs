use std::sync::{Arc, Mutex};

use rayon::prelude::*;

mod utils;
use utils::{Args, parse_args};

mod grid;
#[allow(unused_imports)]
use grid::{Grid, GridInfo, print_grid, print_grids};

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

    fn does_prefix_exist(&self, prefix: &str, top_row: usize) -> bool {
        let prefix_bytes = prefix.as_bytes();
        // TODO: is there a way to avoid collecting? no need to get an entire list
        let prefixes: Vec<String> = self.dictionary.get_trie(top_row).predictive_search(prefix_bytes).collect();
        return !prefixes.is_empty();
    }
    
    fn are_cols_valid(&self, grid: &Grid, grid_info: &GridInfo, check_cols_for_words: bool) -> bool {
        for col_index in 0..self.side_len {
            let mut col_str = String::new();
            for row_index in &grid.grid {
                col_str.push(self.dictionary.get_word(row_index).chars().nth(col_index).unwrap());
            }
            if !self.does_prefix_exist(&col_str, grid_info.starting_row) {
                return false;
            }
    
            // Column shouldn't contain row word
            if check_cols_for_words {
                // TODO: maybe use a hashmap for O(1) lookup
                if grid.grid.contains(&col_index) {
                    return false;
                }
            }
        }
        return true;
    }

    // TODO: give a nice explaination
    fn solve_row_chunks(
        &self,
        grid_info: &GridInfo,
        top_bank: &Vec<Grid>,
        bottom_bank: &Vec<Grid>,
    ) -> Vec<Grid> {
        // TODO: maybe give each thread it's own valid_grids vector 
        // and consilidate them at the end to avoid locks
        let valid_grids: Arc<Mutex<Vec<Grid>>> = Arc::new(Mutex::new(Vec::new()));
        let chunk_len = top_bank[0].len();
        let num_chunks = chunk_len * 2;
        
        // TODO: avoid repeated words        
        // This is tough with the divide and conquer approach

        // Iterate over top bank in parallel
        top_bank
        .par_iter()
        .for_each(|top_word_chunk| {
            let mut grid: Grid = Grid::new();
            // Fill top chunk of the grid
            grid.replace_range(0..chunk_len, top_word_chunk);
            
            // NOTE: This can be skipped during the first execution (when top_word_chunk = word_list),
            // if you do some preprocessing to make sure every word can go in every column
            if !self.are_cols_valid(&grid, grid_info, false) {
                return;
            }
            
            // Iterate over bottom bank in parallel
            bottom_bank
            .par_iter()
            .for_each(|bottom_word_chunk| {
                let mut grid_clone = grid.clone();
                
                // Fill bottom chunk of the grid
                grid_clone.replace_range(chunk_len..num_chunks, bottom_word_chunk);

                // Exit early if grid invalid
                if !self.are_cols_valid(&grid_clone, grid_info, false) {
                    return;
                }
                // Print complete grids
                if grid_clone.len() == self.side_len{
                    // println!("{}", grid_clone);
                    print_grid(&grid_clone, &self.dictionary);
                }
                valid_grids.lock().unwrap().push(grid_clone.clone());
            });
        });

        return Arc::try_unwrap(valid_grids)
        .expect("Failed to unwrap Arc")
        .into_inner()
        .expect("Failed to unlock Mutex");
    }

    fn divide_and_conquer(
        &self,
        grid_info: &GridInfo,
    ) -> Vec<Grid> {
        if grid_info.ending_row - grid_info.starting_row <= 1 {
            // Make a list of all indexes like this: [[0], [1], [2], ..., [dict.len()]]
            let word_indexes: Vec<Grid> = (0..self.dictionary.get_word_list().len())
            .map(|i| Grid{grid: vec![i]}).collect();
            let ret = self.solve_row_chunks(grid_info, &word_indexes, &word_indexes);
            return ret;
        }

        // Split grid into two
        let mid_row = (grid_info.starting_row + grid_info.ending_row) / 2;
        let top_grid = GridInfo {
            starting_row: grid_info.starting_row,
            ending_row: mid_row,
        };
        let bottom_grid = GridInfo {
            starting_row: mid_row + 1,
            ending_row: grid_info.ending_row,
        };

        // Run the two halves in parallel
        let (top_result, bottom_result): (Vec<Grid>, Vec<Grid>) = rayon::join(
            || self.divide_and_conquer(&top_grid),
            || self.divide_and_conquer(&bottom_grid),
        );

        // Combine the results
        return self.solve_row_chunks(grid_info, &top_result, &bottom_result)

    }

    fn solve(&self) -> Vec<Grid>{
        // TODO: can I avoid repeated transpose solutions?
        // TODO: maybe just use a fixed-size array instead of Vec --> No bounds checking
    
        let initial_grid_info = GridInfo::new(0, self.side_len - 1);
        return self.divide_and_conquer(&initial_grid_info);
    }
}


fn main() {
    let args: Args = parse_args();

    let solver: Solver = Solver::new(
        args.side_len, 
        Dict::new(args.dict_path.clone(), args.side_len)
    );
    println!("Dictionary loaded with {} words.", solver.dictionary.get_word_list().len());
    // println!("{:?}", solver.dictionary.get_trie(0).predictive_search("").collect::<Vec<String>>());

    solver.solve();
}

