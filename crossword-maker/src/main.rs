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
    
    fn are_cols_valid(&self, grid: &Grid, grid_info: &GridInfo) -> bool {
        for col_index in 0..self.side_len {
            let mut col_str = String::new();
            for row_index in &grid.grid {
                col_str.push(self.dictionary.get_word(row_index).chars().nth(col_index).unwrap());
            }

            if !self.does_prefix_exist(&col_str, grid_info.starting_row) {
                return false;
            }

            // TODO: don't allow words that are already used horizontally
            // Maybe I just check at the end?
            // Maybe I can implement the Trie on my own to be able to delete

            // if grid.has_overlap(&Grid::from_vec(col_str)){
            //     return false;
            // }
    
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
        let num_rows = grid_info.ending_row - grid_info.starting_row + 1;
        let chunk_len = (num_rows + 1) / 2;

        // This is just for the bottom row of an odd size grid
        // Technically it can be skipped with some preprocessing
        if num_rows == 1 {
            top_bank
            .par_iter()
            .for_each(|chunk| {
                let mut grid: Grid = Grid::new();
                
                // Fill bottom chunk of the grid
                grid.replace_range(0..1, chunk);

                // Exit early if grid invalid
                if !self.are_cols_valid(&grid, grid_info) {
                    return;
                }

                valid_grids.lock().unwrap().push(grid.clone());
            });
        } 
        else {
            top_bank
            .par_iter()
            .for_each(|top_word_chunk| {
                let mut grid: Grid = Grid::new();
                // Fill top chunk of the grid
                grid.replace_range(0..chunk_len, top_word_chunk);
                
                // NOTE: we know this top_word_chunk placement is valid
                // If it wasn't, it would have already been filtered out.
                // The only time this isn't true is when top_word_chunk is one row long
                // But with a large each dictionary, it's unlikely it's not valid.
                
                // Iterate over bottom bank in parallel
                bottom_bank
                .par_iter()
                .for_each(|bottom_word_chunk| {
                    
                    // We can't reuse words from top chunk
                    // TODO: make this more efficient?
                    // or maybe only check at the end since this doesn't prune that many branches?
                    if grid.has_overlap(bottom_word_chunk) {
                        return; 
                    }

                    let mut grid_clone = grid.clone();
                    
                    // Fill bottom chunk of the grid
                    grid_clone.replace_range(chunk_len..num_rows, bottom_word_chunk);
    
                    // Exit early if grid invalid
                    if !self.are_cols_valid(&grid_clone, grid_info) {
                        return;
                    }
                    // Print complete grids
                    if grid_clone.len() == self.side_len{
                        print_grid(&grid_clone, &self.dictionary);
                    }
    
                    valid_grids.lock().unwrap().push(grid_clone.clone());
                });
            });
        }
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
    
        let initial_grid_info = GridInfo::new(0, self.side_len - 1);
        return self.divide_and_conquer(&initial_grid_info);
    }
}


fn main() {
    // TODO: can I avoid repeated transpose solutions?
    // TODO: add unit tests and timing benchmarks

    let args: Args = parse_args();

    let solver: Solver = Solver::new(
        args.side_len, 
        Dict::new(args.dict_path.clone(), args.side_len)
    );
    println!("Dictionary loaded with {} words.", solver.dictionary.get_word_list().len());
    // println!("{:?}", solver.dictionary.get_trie(0).predictive_search("").collect::<Vec<String>>());

    solver.solve();
}

