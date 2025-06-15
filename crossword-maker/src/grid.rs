use std::fmt;

#[derive(Clone)]
#[derive(Debug)]
pub struct Grid{
    pub grid: Vec<String>,
    // pub starting_row: usize,
    // pub ending_row: usize,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            grid: Vec::new(),
            // starting_row: starting_row,
            // ending_row: ending_row,
        }
    }

    pub fn len(&self) -> usize {
        return self.grid.len()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_dashes = 2 * self.grid[0].len() + 1;
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