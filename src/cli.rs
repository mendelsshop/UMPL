use std::process::exit;

#[derive(Debug)]
pub struct ParsedArgs {
    pub repl: bool,   // inerative mode
    pub file: String, // file to read/write
    pub force: bool,  // if true, overwrites file
}

impl ParsedArgs {
    fn parsed_args(repl: bool, file: String) -> ParsedArgs {
        ParsedArgs {
            repl,
            file,
            force: false,
        }
    }
}

pub fn get_string_args(args: &Vec<String>) -> (usize, ParsedArgs) {
    let mut to_return = ParsedArgs::parsed_args(false, String::from(""));
    let mut index: usize = 1; // start at 1 because index  0 is the program name
    if args.len() < 2 {
        // if there are no arguments run in repl mode with no file
        return (0, ParsedArgs::parsed_args(true, String::from("")));
    } else if args[1].ends_with(".umpl") {
        // make sure it's a .umpl file
        to_return.file = args[1].clone(); // if it is, then set file to the file name
        to_return.repl = false; // and set repl to false
        index += 1; // and increment index
    } else {
        for (arg_index, arg) in args[index..].iter().enumerate() {
            if arg.starts_with('-') {
                // if it starts with a dash
                index += arg_index; // then add the args index to the current index
                break; // and break
            } else {
                println!("{}", usage()); // if not a flag, then its not one of the args we want so print usage and exit
                exit(1);
            }
        }
    };
    (index, to_return)
}

pub fn get_dash_args(args: &[String], start_index: usize, args_struct: &mut ParsedArgs) {
    for arg in args[start_index..].iter() {
        // for each arg after the start index
        if arg.starts_with('-') {
            // if it starts with a dash check if its a correct flag and set the appropriate field if not print usage and exit
            for char_part_arg in arg.chars().skip(1) {
                if ['r', 'i'].contains(&char_part_arg) {
                    args_struct.repl = true;
                } else if ['f'].contains(&char_part_arg) {
                    args_struct.force = true;
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

fn usage() -> String {
    String::from(
        "Usage: umpl [File] [OPTIONS]
    OPTIONS: 
    -r, -i: interactive mode
    -h: help
    -v: version
    -f: force",
    )
}
