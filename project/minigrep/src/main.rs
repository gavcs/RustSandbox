/// Filename: minigrep
/// this project works as grep, but written in rust. This means that to run this project, the user must enter:
///     $ cargo run -- search_string file_path
/// The search_string will be the substring that the user is looking for. The file_path is the path to the file
/// that minigrep will look for the search_string in.
/// 
/// @author: Gavin McConnell (gavcs)
/// @date 7/17/2024
/// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// /// ///

use std::env;   // used to read command line arguments
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}