use std::fmt;

#[derive(Clone)]
#[derive(Debug)]
pub struct Grid{
    pub num_rows: usize,
    pub num_cols: usize,
    pub grid: Vec<String>,
    // pub starting_row: usize,
    // pub ending_row: usize,
}

impl Grid {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        Grid {
            num_rows: num_rows,
            num_cols: num_cols,
            grid: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
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

pub fn print_grids(grids: &Vec<Grid>) {
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