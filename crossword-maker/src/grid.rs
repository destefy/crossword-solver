use std::fmt;
use std::ops::Index;
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
    pub grid: Vec<String>
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
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_dashes = 2 * self.grid.len() + 1;
        println!("{}", "-".repeat(num_dashes));
        for row in &self.grid {
            write!(f, "|")?;
            for ch in row.chars() {
                write!(f, "{}|", ch)?;
            }
            writeln!(f)?;
        }
        println!("{}", "-".repeat(num_dashes));
        Ok(())
    }
}

    #[allow(dead_code)]
pub fn print_grids(grids: &Vec<Grid>) {
    if grids.is_empty() {
        println!("No solutions found.");
        return;
    }
    for grid in grids {
        print!("{}", grid);
    }
}