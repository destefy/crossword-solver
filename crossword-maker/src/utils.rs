use std::env;

pub struct Args {
    pub num_rows: usize,
    pub num_cols: usize,
    pub dict_path: String,
}

pub fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Usage: {} <num_rows> <num_cols> <dict_path>", args[0]);
    }

    let num_rows = args[1].parse::<usize>().expect("Invalid number of rows");
    let num_cols = args[2].parse::<usize>().expect("Invalid number of columns");
    let dict_path = args[3].clone();

    Args {
        num_rows,
        num_cols,
        dict_path,
    }
}