mod input;
mod dial;
mod interpreter;
use std::env;

fn main() {
    let dial_min = 0;
    let dial_max = 99;
    let dial_start = 50;

    let is_verbose = parse_args(env::args());

    let instructions: Vec<(char, i32)> = input::get_instructions();
    let stops: Vec<(&(char, i32), i32)> = dial::dial(
        dial_start,
        dial_min,
        dial_max,
        &instructions);

    let password = interpreter::interpret_password(
        &(dial_min, dial_max, dial_start),
        &stops,
        is_verbose);

    println!("Password: {}", password);
}

fn parse_args(args: env::Args) -> bool {
    let mut is_verbose: bool = false;
    for argument in args {
        if argument == "--verbose" || argument == "-v" {
            is_verbose = true;
        }
    }

    return is_verbose;
}
