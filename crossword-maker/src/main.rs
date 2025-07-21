mod utils; 
use crossword_maker::run_solver;

#[allow(dead_code)]
fn main() {
    let args: utils::Args = utils::parse_args();
    let print_sols = true;
    run_solver(args.side_len, &args.dict_path, print_sols);
}

