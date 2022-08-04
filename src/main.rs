use log::info;
use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    path::Path,
    process::exit,
};
use umpl::{cli, error, eval::Eval, lexer::Lexer, parser::Parser};

fn main() {
    let args: Vec<String> = env::args().collect(); // get the args
    let (index, mut parsed_args): (usize, cli::ParsedArgs) = cli::get_string_args(&args); // get the ile name args and the index of the firrst flag
    if index != 0 {
        // if there are any args after the program name parse them
        cli::get_dash_args(&args, index, &mut parsed_args);
    }
    if parsed_args.log {
        log4rs::init_file("log.yaml", log4rs::config::Deserializers::default()).unwrap();
        info!("Starting up...");
    }
    let mut full_repl: Vec<String> = Vec::new(); // create a vector to hold the lines of the repl just in case we need to write it to a file
    if parsed_args.repl {
        // if we are in repl mode
        let mut current_repl: String = String::new(); // create a string to hold the current line of the repl
        'l: loop {
            let mut input = String::new();
            print!(">> "); // print the prompt
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap(); // read the input
            full_repl.push(input.to_string()); // add the input to the the text of the repl so we can write it to a file
            if input.trim() == "exit" {
                // if the input is exit, then exit
                info!("Exiting...");
                if !parsed_args.file.is_empty() {
                    // if we are in repl mode and we have a file to write to
                    if Path::new(&parsed_args.file).exists() && !parsed_args.force {
                        // if the file exists and we are not forcing it to overwrite
                        print!("Do you want to overwrite the {}? (y/n): ", parsed_args.file); // ask the user if they want to overwrite the file
                        io::stdout().flush().unwrap();
                        let mut y_or_n: String = String::new();
                        io::stdin().read_line(&mut y_or_n).unwrap();
                        if y_or_n == "n" {
                            // if the user does not want to overwrite the file exit
                            exit(1);
                        }
                    }
                    let mut file: File = File::create(parsed_args.file)
                        .expect("Error encountered while creating file!"); // create/open the file
                    for mut line in full_repl {
                        line.push('\n');
                        file.write_all(line.as_bytes())
                            .expect("Error encountered while writing to file!");
                        // write the repl to the file
                    }
                }
                break 'l;
            } else if input.trim() == "run" {
                run(current_repl.to_string()); // run the current repl
                input.clear();
            } else {
                current_repl.push_str(&input); // add the input to the current line of the repl
            }
        }
    } else {
        // if we are not in repl mode ie we are reading a file
        let mut file: File = File::open(&parsed_args.file).unwrap(); // open the file
        let mut contents: String = String::new(); // create a string to hold the contents of the file
        match file.read_to_string(&mut contents) {
            Ok(contents) => contents,
            Err(_) => {
                error::error(0, "could not read file");
            }
        }; // read the file into the string
        run(contents); // run the file
    }
}

fn run(line: String) {
    let lexer: Lexer = Lexer::new(line);
    let mut parsed: Parser = Parser::new(lexer.scan_tokens());
    Eval::new(parsed.parse());
}
