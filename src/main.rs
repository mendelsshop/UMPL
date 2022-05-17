use std::{env::{self}, process::exit};
#[derive(Debug)]
struct ParsedArgs {
    repl: bool,
    file: String,
}

impl ParsedArgs {
    fn parsed_args(repl: bool, file: String) -> ParsedArgs {
        ParsedArgs {
            repl,
            file,
        }
    }
    fn from_dashes(&self, repl: bool)  { 
        self.repl;
        println!("{}", self.repl);
    }
}
fn main() {
    let mut args: Vec<String> = env::args().collect();
    println!("{args:?} {}", args.len());
    let (index, mut parsed_args) = get_string_args(&args);
    println!("{index}");
    if index != 0 {
        get_dash_args(&args, index, &mut parsed_args);
    }
    println!("{:?}",parsed_args);

    
}

fn usage() -> String {
    String::from("Usage: umpl [File] [OPTIONS]\n
    OPTIONS: 
    -r, -i: interactive mode")
}
fn get_string_args(args: & Vec<String>) -> (usize, ParsedArgs) {
    let mut to_return: ParsedArgs;
    let mut index: usize = 0;
    if args.len() < 1 {
        return (0, ParsedArgs::parsed_args(true, String::from("")));
    } else if args[1].ends_with(".umpl") {
        to_return = ParsedArgs::parsed_args(
            false,
            args[1].clone(),
        );
        index = 2;
    } else {
        println!("{}", usage());
        exit(1);
    }

    for (arg_index, arg) in args[index..].iter().enumerate() {
        if arg.starts_with('-') {
            index = arg_index + index;
            break;
        } else {
            println!("{}",usage());
            exit(1);
        }
    };
    (index, to_return)
}

fn get_dash_args(args: & Vec<String>, start_index: usize, args_struct: &mut ParsedArgs) {
    for arg in args[start_index..].iter() {
        if arg.starts_with('-') {
            for char_part_arg in arg.chars().skip(1) {
                if ['r', 'i'].contains(&char_part_arg) {
                    println!("in ri");
                    args_struct.repl = true;
                } else {
                    println!("{}", usage());
                    exit(1);
                }
            }
        } else {
            println!("{}", usage());
            exit(1);
        }
    }
}
