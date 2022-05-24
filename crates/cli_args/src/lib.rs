use std::process::exit;

#[derive(Debug)]
pub struct ParsedArgs {
    pub repl: bool,
    pub file: String,
    pub force: bool,
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

pub fn get_string_args(args: & Vec<String>) -> (usize, ParsedArgs) {
    let mut to_return: ParsedArgs;
    let mut index: usize = 0;
    to_return = ParsedArgs::parsed_args(
        false,
        String::from(""),
    );
    index += 1;
    if args.len() < 2 {
        return (0, ParsedArgs::parsed_args(true, String::from("")));
    } else if args[1].ends_with(".umpl") {
        to_return.file = args[1].clone();
        to_return.repl = false;
        index += 1;
    } else {
    for (arg_index, arg) in args[index..].iter().enumerate() {
        println!("{}", arg);
        if arg.starts_with('-') {
            index = arg_index + index;
            break;
        } else {
            println!("{} in get_string_args",usage());
            exit(1);
        }
    }; };
    (index, to_return)
}

pub fn get_dash_args(args: & Vec<String>, start_index: usize, args_struct: &mut ParsedArgs) {
    for arg in args[start_index..].iter() {
        if arg.starts_with('-') {
            for char_part_arg in arg.chars().skip(1) {
                if ['r', 'i'].contains(&char_part_arg) {
                    println!("in ri");
                    args_struct.repl = true;
                } else if ['f'].contains(&char_part_arg) {
                    println!("in f");
                    args_struct.force = true;
                } else if ['h'].contains(&char_part_arg) {
                    println!("{}", usage());
                    exit(1);
                } else {
                    println!("{} get_dash_args there is dash", usage());
                    exit(1);
                }
            }
        } else {
            println!("{} get_dash_args no dash", usage());
            exit(1);
        }
    }
}

fn usage() -> String {
    String::from("Usage: umpl [File] [OPTIONS]\n
    OPTIONS: 
    -r, -i: interactive mode
    -h: help
    -v: version
    -f: force")
}

// let mut args: Vec<String> = env::args().collect();
// println!("{args:?} {}", args.len());
// let (index, mut parsed_args) = get_string_args(&args);
// println!("{index}");
// if index != 0 {
//     get_dash_args(&args, index, &mut parsed_args);
// }
// println!("{:?}",parsed_args);

// turn the above into a function