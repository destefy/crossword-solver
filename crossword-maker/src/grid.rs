use std::ops::Index;

pub use crate::dict::Dict;

#[derive(Clone)]
#[derive(Debug)]

pub struct GridInfo{
    pub starting_row: usize,
    pub ending_row: usize,
}

impl GridInfo {
    pub fn new(starting_row: usize, ending_row: usize) -> Self {
        GridInfo {
            starting_row: starting_row,
            ending_row: ending_row,
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Grid{
    pub grid: Vec<usize>
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            grid: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.grid.len()
    }

    // Replace (or extend) the grid in range [start, end) with new_elements
    pub fn replace_range(&mut self, range: std::ops::Range<usize>, new_elements: &Grid) {
        // TODO: maybe allow new_elements to be a String/index rather than a Grid (for the basecase calls)
        assert!(range.end - range.start == new_elements.len(), "Range length must match new elements length");
        
        if self.len() < range.end {
            self.grid.extend(new_elements.grid.clone());
            return;
        }
        // Replace elements in the specified range with new_elements
        self.grid.splice(range, new_elements.clone().grid);
    }
}

// Implement the Index trait for Grid
impl Index<usize> for Grid {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

pub fn print_grid(grid: &Grid, dictionary: &Dict) {
    let num_dashes = 2 * &dictionary.get_word_list()[0].len() + 1;
    println!("\n{}", "-".repeat(num_dashes));
    for row_index in &grid.grid {
        let row_word: &String = dictionary.get_word(row_index);
        print!("|");
        for ch in row_word.to_string().chars() {
            print!("{}|", ch);
        }
        println!();
    }
    print!("{}", "-".repeat(num_dashes));
}

#[allow(dead_code)]
pub fn print_grids(grids: &Vec<Grid>, dictionary: &Dict) {
    if grids.is_empty() {
        println!("No solutions found.");
        return;
    }
    for grid in grids {
        print_grid(grid, dictionary);
    }
}