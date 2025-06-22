// use std::sync::{Arc, Mutex};

// use rayon::prelude::*;

mod utils;
use utils::{Args, parse_args};

mod grid;
#[allow(unused_imports)]
use grid::{Grid, GridInfo, print_grids};

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
        for col in 0..self.side_len {
            let mut col_str = String::new();
            for row in 0..grid.len() {
                col_str.push(grid[row].chars().nth(col).unwrap());
            }
            if !self.does_prefix_exist(&col_str, grid_info.starting_row) {
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
    
    // fn is_grid_valid(&self, grid: &Grid) -> bool {
    //     if grid.len() != self.side_len {
    //         return false;
    //     }
    //     if !self.are_cols_valid(grid, true) {
    //         return false;
    //     }
    //     return true;
    // }

    // fn backtrack(
    //     &self,
    //     grid: &Grid,
    //     iter: usize, 
    //     solution_grids: &Arc<Mutex<Vec<Grid>>>,
    // ){    
    //     if self.is_grid_valid(&grid) {
    //         println!("{}", grid);
    //         let mut vec = solution_grids.lock().unwrap();
    //         vec.push(grid.clone());
    //         return;
    //     }
    
    //     self.dictionary
    //     .get_word_list(iter)
    //     .par_iter() // Use parallel iterator
    //     .filter(|word| !grid.grid.contains(*word)) // Exclude used words
    //     .for_each(|word| {
    //         if !self.are_cols_valid(&grid, false) {
    //             return;
    //         }
    
    //         let mut grid_clone = grid.clone();
    //         grid_clone.grid.push(word.clone());
    
    //         // Recursive call
    //         self.backtrack(&grid_clone, iter, solution_grids);
    
    //         grid_clone.grid.pop();
    //     });
    // }

    fn solve_row_chunks(
        &self,
        grid_info: &GridInfo,
        top_bank: &Vec<Grid>,
        bottom_bank: &Vec<Grid>,
    ) -> Vec<Grid> {
        let mut grid: Grid = Grid::new();
        let chunk_len = top_bank[0].len();
        let mut valid_grids: Vec<Grid> = Vec::new();

        for top_word_chunk in top_bank {
            // Fill top chunk of the grid
            grid.replace_range(0..chunk_len, top_word_chunk);
            if !self.are_cols_valid(&grid, grid_info, false) {
                continue;
            }
            for bottom_word_chunk in bottom_bank {
                // Fill bottom chunk of the grid
                grid.replace_range(0..chunk_len, bottom_word_chunk);
                if !self.are_cols_valid(&grid, grid_info, false) {
                    continue;
                }
                valid_grids.push(grid.clone());
            }
            // Remove the bottom chunk of the grid
            grid.grid.truncate(chunk_len);
        }

        return valid_grids;
    }

    fn divide_and_conquer(
        &self,
        grid_info: &GridInfo,
    ) -> Vec<Grid> {

        if grid_info.ending_row - grid_info.starting_row <= 1 {
            let word_bank = self.dictionary.get_word_list();
            return self.solve_row_chunks(grid_info, word_bank, word_bank);
        }

        // Split grid into two
        let top_grid = GridInfo {
            starting_row: grid_info.starting_row,
            ending_row: grid_info.ending_row / 2,
        };
        let bottom_grid = GridInfo {
            starting_row: grid_info.ending_row / 2 + 1,
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
    // println!("{:?}", solver.dictionary.get_trie(2).predictive_search("").collect::<Vec<String>>());

    solver.solve();
}

