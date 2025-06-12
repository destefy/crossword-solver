use std::fmt;
use trie_rs::TrieBuilder;

#[derive(Clone)]
struct Grid{
    num_rows: usize,
    num_cols: usize,
    grid: Vec<String>,
}

impl Grid {
    fn new(num_rows: usize, num_cols: usize) -> Self {
        Grid {
            num_rows: num_rows,
            num_cols: num_cols,
            grid: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        return self.grid.len()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            write!(f, "|")?;
            for ch in row.chars() {
                write!(f, "{}|", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn print_grids(grids: &Vec<Grid>) {
    if grids.is_empty() {
        println!("No solutions found.");
        return;
    }
    let num_dashes = 2 * grids[0].num_cols + 1;
    for grid in grids {
        println!("{}", "-".repeat(num_dashes));
        print!("{}", grid);
        println!("{}", "-".repeat(num_dashes));
    }
}

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
    let mut builder = TrieBuilder::new();
    builder.push("ABC");
    builder.push("DEF");
    builder.push("ZZT");
    builder.push("GHI");
    builder.push("BEH");
    builder.push("CFI");
    builder.push("ADG");
    let dictionary = builder.build();

    let num_rows = 3;
    let solutions = solve(&dictionary, num_rows, num_rows);
    print_grids(&solutions);
}
