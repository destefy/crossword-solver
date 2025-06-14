mod grid;
use grid::Grid;
use grid::print_grids;

mod dict;
use dict::load_into_dict;

use rayon::prelude::*;
use std::sync::{Arc, Mutex};

fn does_prefix_exist(dictionary: &trie_rs::Trie<u8>, prefix: &str) -> bool {
    let prefix_bytes = prefix.as_bytes();
    let prefixes: Vec<String> = dictionary.predictive_search(prefix_bytes).collect();
    return !prefixes.is_empty();
}

fn are_cols_valid(dictionary: &trie_rs::Trie<u8>, grid: &Grid) -> bool {
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

fn is_grid_valid(dictionary: &trie_rs::Trie<u8>, grid: &Grid) -> bool {
    if grid.len() != grid.num_rows {
        return false;
    }
    if !are_cols_valid(dictionary, grid) {
        return false;
    }
    return true;
}

fn backtrack(dictionary: &trie_rs::Trie<u8>, grid: &Grid, solution_grids: &Arc<Mutex<Vec<Grid>>>){    
    if is_grid_valid(&dictionary, &grid) {
        println!("Found a valid grid: {:?}", grid.grid);
        let mut vec = solution_grids.lock().unwrap();
        vec.push(grid.clone());
        return;
    }
    
    dictionary
    .predictive_search("")
    .collect::<Vec<String>>()
    .par_iter() // Use parallel iterator
    .for_each(|word| {
        // println!("Trying grid: {:?}", grid);

        if !are_cols_valid(dictionary, &grid) {
            return;
        }

        let mut grid_clone = grid.clone();

        grid_clone.grid.push(word.clone());

        backtrack(dictionary, &grid_clone, solution_grids);

        grid_clone.grid.pop();
    });
}

fn solve(dictionary: &trie_rs::Trie<u8>, num_rows: usize, num_cols: usize) -> Vec<Grid>{
    // TODO: maybe just use a fixed-size array instead of Vec --> No bounds checking
    // TODO: don't store rows and cols in Grid

    let mut initial_grid = Grid::new(num_rows, num_cols);
    let mut solution_grids: Arc<Mutex<Vec<Grid>>> = Arc::new(Mutex::new(Vec::new()));
    backtrack(dictionary, &mut initial_grid, &mut solution_grids);
    return Arc::try_unwrap(solution_grids)
        .expect("Failed to unwrap Arc")
        .into_inner()
        .expect("Failed to unlock Mutex");
}


fn main() {
    let input_file_name: String = "word_banks/four_letter_words.txt".to_string();
    let dictionary = load_into_dict(input_file_name);
    println!("Dictionary loaded with {} words.", dictionary.predictive_search("").collect::<Vec<String>>().len());

    let num_rows = 4;
    let solutions = solve(&dictionary, num_rows, num_rows);
    print_grids(&solutions);
}
