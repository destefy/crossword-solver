use std::sync::{Arc, Mutex};

use rayon::prelude::*;

mod utils;
use utils::{Args, parse_args};

mod grid;
use grid::{Grid, print_grids};

mod dict;
use dict::Dict;


fn does_prefix_exist(dictionary: &Dict, prefix: &str) -> bool {
    let prefix_bytes = prefix.as_bytes();
    // TODO: is there a way to avoid collecting? no need to get an entire list
    let prefixes: Vec<String> = dictionary.get_trie().predictive_search(prefix_bytes).collect();
    return !prefixes.is_empty();
}

fn are_cols_valid(dictionary: &Dict, grid: &Grid) -> bool {
    for col in 0..grid.num_cols {
        let mut col_str = String::new();
        for row in 0..grid.len() {
            col_str.push(grid.grid[row].chars().nth(col).unwrap());
        }
        if !does_prefix_exist(dictionary, &col_str) {
            return false;
        }
    }
    return true;
}

fn is_grid_valid(dictionary: &Dict, grid: &Grid) -> bool {
    if grid.len() != grid.num_rows {
        return false;
    }
    if !are_cols_valid(dictionary, grid) {
        return false;
    }
    return true;
}

fn backtrack(
    dictionary: &Dict, 
    grid: &Grid, 
    solution_grids: &Arc<Mutex<Vec<Grid>>>,
){    
    if is_grid_valid(&dictionary, &grid) {
        println!("Found a valid grid: {:?}", grid.grid);
        let mut vec = solution_grids.lock().unwrap();
        vec.push(grid.clone());
        return;
    }
    

    dictionary
    .get_word_list()
    .par_iter() // Use parallel iterator
    .filter(|word| !grid.grid.contains(*word)) // Exclude used words
    .for_each(|word| {
        if !are_cols_valid(dictionary, &grid) {
            return;
        }

        let mut grid_clone = grid.clone();
        grid_clone.grid.push(word.clone());

        // Recursive call
        backtrack(dictionary, &grid_clone, solution_grids);

        grid_clone.grid.pop();
    });
}

fn solve(dictionary: &Dict, num_rows: usize, num_cols: usize) -> Vec<Grid>{
    // TODO: maybe just use a fixed-size array instead of Vec --> No bounds checking
    // TODO: don't store rows and cols in Grid

    let mut initial_grid = Grid::new(num_rows, num_cols);
    let mut solution_grids: Arc<Mutex<Vec<Grid>>> = Arc::new(Mutex::new(Vec::new()));

    backtrack(dictionary, &mut initial_grid, &mut solution_grids);
    return Arc::try_unwrap(solution_grids)
        .expect("Failed to unwrap Arc")
        .into_inner()
        .expect("   Failed to unlock Mutex");
}


fn main() {
    let args: Args = parse_args();

    // let input_file_name: String = "word_banks/four_letter_words.txt".to_string();
    let dictionary = Dict::new(args.dict_path);
    println!("Dictionary loaded with {} words.", dictionary.get_word_list().len());

    let solutions = solve(&dictionary, args.num_rows, args.num_cols);
    print_grids(&solutions);
}

