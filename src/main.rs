use image_sorter::*;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let path = match parse_path_args() {
        Ok(p) => p,
        Err(err) => panic!("Problem parsing path argrs {:?}", err),
    };

    let _res = match process_files(path) {
        Ok(_) => println!("Sorting completed sucessfully!"),
        Err(err) => panic!("{}", err),
    };
}
