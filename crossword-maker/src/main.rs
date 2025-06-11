// struct Grid {
//     grid: <Vec<String>,
// }

// impl Grid {
//     fn new(num_rows: usize, num_cols: usize) -> Self {
//         let grid = vec![String::new(); num_rows];
//         Grid { grid }
//     }
// }

struct Solver{
    solution_grids: Vec<Vec<String>>,
    num_rows: usize,
    num_cols: usize,
    dictionary: Vec<String>,
}

impl Solver {
    fn new(num_rows: usize, num_cols: usize, dictionary: Vec<String>) -> Self {
            Solver {
                solution_grids: Vec::new(),
                num_rows: num_rows,
                num_cols: num_cols,
                dictionary: dictionary,
            }
        }

    fn does_prefix_exist(&self, prefix: &str) -> bool {
        self.dictionary.iter().any(|word| word.starts_with(prefix))
    }

    fn are_cols_valid(&self, grid: &Vec<String>) -> bool {
        for col in 0..self.num_cols {
            let mut col_str = String::new();
            for row in 0..self.num_rows {
                col_str.push(grid[row].chars().nth(col).unwrap());
            }
            if !self.does_prefix_exist(&col_str) {
                return false;
            }
        }
        return true;
    }

    fn backtrack(&mut self, mut grid: Vec<String>) {
        if grid.len() == self.num_rows {
            if self.are_cols_valid(&grid) {
                self.solution_grids.push(grid);
            }
            return;
        }
        
        let dictionary_iter = self.dictionary.iter();
        for word in dictionary_iter {
            if !self.are_cols_valid(&grid) {
                continue;
            }
            let mut new_grid = grid.clone();
            new_grid.push(word.clone());

            self.backtrack(new_grid);

            grid.pop();
        }
    }

    fn solve(&mut self) -> &Vec<Vec<String>>{
        // TODO: maybe just use a fixed-size array instead of Vec
        let initial_grid = Vec::new();
        self.backtrack(initial_grid);
        return &self.solution_grids;
    }

}


fn main() {
    let dictionary = vec![
        String::from("ABC"),
        String::from("DEF"),
        String::from("ZZT"),
        String::from("GHI"),
        String::from("BEH"),
        String::from("CFI"),
        String::from("ADG"),
    ];
    let mut crossword  = Solver::new(3, 3, dictionary);
    let solutions = crossword.solve();
}
