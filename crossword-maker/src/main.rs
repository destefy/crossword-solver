mod grid;
use grid::Grid;
use grid::print_grids;

mod dict;
use dict::load_into_dict;


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

fn backtrack(dictionary: &trie_rs::Trie<u8>, grid: &mut Grid, solution_grids: &mut Vec<Grid>){
    if is_grid_valid(&dictionary, &grid) {
        solution_grids.push(grid.clone());
        return;
    }
    
    for word in dictionary.predictive_search("").collect::<Vec<String>>() {
        if !are_cols_valid(dictionary, &grid) {
            continue;
        }
        grid.grid.push(word.clone());

        backtrack(dictionary, grid, solution_grids);

        grid.grid.pop();
    }
}

fn solve(dictionary: &trie_rs::Trie<u8>, num_rows: usize, num_cols: usize) -> Vec<Grid>{
    // TODO: maybe just use a fixed-size array instead of Vec
    let mut initial_grid = Grid::new(num_rows, num_cols);
    let mut solution_grids: Vec<Grid> = Vec::new();
    backtrack(dictionary, &mut initial_grid, &mut solution_grids);
    return solution_grids;
}


fn main() {
    // let mut builder = TrieBuilder::new();
    // let dictionary = builder.build();\
    let dictionary = load_into_dict("word_list.txt".to_string());

    let num_rows = 4;
    let solutions = solve(&dictionary, num_rows, num_rows);
    print_grids(&solutions);
}
