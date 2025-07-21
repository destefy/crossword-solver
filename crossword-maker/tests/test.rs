use crossword_maker::{run_solver, grid::Grid, dict::Dict};

#[test]
fn test_side_four() {
    let side_len = 4;
    let dict_path = "word_banks/four_letter/test.txt";
    let solution: Vec<Grid> = run_solver(4, dict_path, false);
    let dict: Dict = Dict::new(dict_path.to_string(), side_len);

    assert!(solution.len() == 2);

    // verify rows are valid words
    for grid in &solution {
        for row in grid {
            assert!(row < &dict.get_word_list().len());
            let row_str: &String = dict.get_word(row);
            assert!(dict.get_word_list().contains(row_str));
        }
    }
    // verify columns are valid words
    for grid in &solution {
        for col_index in 0..side_len {
            let mut col_str = String::new();
            for row_index in &grid.grid {
                col_str.push(dict.get_word(row_index).chars().nth(col_index).unwrap());
            }
            assert!(dict.get_word_list().contains(&col_str))
    
        }
    }
}


#[test]
fn test_side_fifteen() {
    let side_len = 15;
    let dict_path = "word_banks/fifteen_letter/test.txt";
    let solution: Vec<Grid> = run_solver(15, dict_path, false);
    let dict: Dict = Dict::new(dict_path.to_string(), side_len);

    assert!(solution.len() == 2);

    // verify rows are valid words
    for grid in &solution {
        for row in grid {
            assert!(row < &dict.get_word_list().len());
            let row_str: &String = dict.get_word(row);
            assert!(dict.get_word_list().contains(row_str));
        }
    }
    // verify columns are valid words
    for grid in &solution {
        for col_index in 0..side_len {
            let mut col_str = String::new();
            for row_index in &grid.grid {
                col_str.push(dict.get_word(row_index).chars().nth(col_index).unwrap());
            }
            assert!(dict.get_word_list().contains(&col_str))
    
        }
    }
}
