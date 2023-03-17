use std::process::exit;

use crate::{
    error,
    token::{Info, Position},
};
pub static mut EASY_MODE: bool = false;
pub static mut TOGGLE_CASE: i32 = 0;
#[derive(PartialEq, Eq, Debug)]
pub struct ParsedArgs {
    pub repl: bool,   // inerative mode
    pub file: String, // file to read/write
    pub force: bool,  // if true, overwrites file
    pub log: bool,    // if true, logs to file
}

impl ParsedArgs {
    const fn new(repl: bool, file: String) -> Self {
        Self {
            repl,
            file,
            force: false,
            log: false,
        }
    }

    pub fn get_info(&'_ self) -> Info<'_> {
        Info::new(
            match &self.file {
                f if f.is_empty() => "<uwkown>",
                f => f,
            },
            Position::new(0, 0),
            Position::new(0, 0),
        )
    }
}

pub fn get_string_args(args: &[String]) -> (usize, ParsedArgs) {
    let mut to_return: ParsedArgs = ParsedArgs::new(false, String::new());
    let mut index: usize = 1; // start at 1 because index  0 is the program name
    if args.len() < 2 {
        // if there are no arguments run in repl mode with no file
        return (0, ParsedArgs::new(true, String::new()));
    } else if args[1].ends_with(".umpl") {
        // make sure it's a .umpl file
        to_return.file = args[1].to_string(); // if it is, then set file to the file name
        to_return.repl = false; // and set repl to false
        index += 1; // and increment index
        let file_len = to_return.file.strip_suffix(".umpl").unwrap().len(); // get the length of the file name without the .umpl
        if args.len() > 2 && args[2] == format!("{file_len}") {
            unsafe {
                EASY_MODE = true;
            }
            index += 1; // and increment index
        } else if args.len() > 2 && args[2] == "show_length" {
            println!("{file_len}"); // print the length of the file name without the .umpl
            exit(1);
        }
    } else {
        to_return.repl = true; // if it's not a .umpl file, then set repl to true
        for (arg_index, arg) in args[index..].iter().enumerate() {
            if arg.starts_with('-') {
                // if it starts with a dash
                index += arg_index; // then add the args index to the current index
                break; // and break
            }
            usage(); // if not a flag, then its not one of the args we want so print usage and exit
        }
    };
    (index, to_return)
}
#[allow(clippy::cast_possible_wrap)]
pub fn get_dash_args(args: &[String], start_index: usize, args_struct: &mut ParsedArgs) {
    args[start_index..].iter().for_each(|arg| {
        // for each arg after the start index
        if arg.starts_with('-') {
            // if it starts with a dash check if its a correct flag and set the appropriate field if not print usage and exit
            for char_part_arg in arg.chars().skip(1) {
                if ['r', 'i'].contains(&char_part_arg) {
                    args_struct.repl = true;
                } else if ['f'].contains(&char_part_arg) {
                    args_struct.force = true;
                } else if ['l'].contains(&char_part_arg) {
                    args_struct.log = true;
                } else if char_part_arg == 'e' {
                    unsafe {
                        EASY_MODE = false;
                    }
                    let num = match &args_struct.file {
                        // TODO: use rand
                        file if file.is_empty() => 0,
                        file => file.len(),
                    };
                    unsafe { TOGGLE_CASE = num as i32 };
                } else if char_part_arg == 't' {
                    let number: i32 = arg.split_once('=').map_or_else(
                        || error::error(args_struct.get_info(), "option t requires an =number"),
                        |n| match n.1.parse() {
                            Ok(value) => value,
                            Err(error) => error::error(args_struct.get_info(), error),
                        },
                    );
                    unsafe {
                        TOGGLE_CASE = number;
                    }
                    break;
                } else {
                    usage();
                }
            }
        } else {
            usage();
        }
    });
}

fn usage() {
    println!("Usage: umpl [File] [OPTIONS]
\t\tOPTIONS: 
\t-r, -i: interactive mode
\t-h: help
\t-f: force
\t-t=number: toggle case");
    unsafe {
        if EASY_MODE {
            exit(0)
        } else {
            error::stackoverflow();
        }
    }
    
}
