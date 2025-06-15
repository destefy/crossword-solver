use std::env;

pub struct Args {
    pub side_len: usize,
    pub dict_path: String,
}

pub fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: {} <side_len> <dict_path>", args[0]);
    }

    let side_len = args[1].parse::<usize>().expect("Invalid side length");
    let dict_path = args[2].clone();

    Args {
        side_len,
        dict_path,
    }
}