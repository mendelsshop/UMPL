use cli_args::{self};
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (index, mut parsed_args) = cli_args::get_string_args(&args);
    if index != 0 {
        cli_args::get_dash_args(&args, index, &mut parsed_args);
    }
    let mut full_repl: Vec<String> = Vec::new();
    if parsed_args.repl {
        loop {
            let mut input = String::new();
            print!(">> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            full_repl.push(input.to_string());
            if input == "exit" {
                break;
            }
            println!("{}", input);
        }
    } else {
        println!("{}", parsed_args.file);
    }

    if parsed_args.repl && !parsed_args.file.is_empty() {
        if Path::new(&parsed_args.file).exists() && !parsed_args.force{
            print!("Do you want to overwrite the {}? (y/n): ", parsed_args.file);
            io::stdout().flush().unwrap();
            let mut y_or_n = String::new();
            io::stdin().read_line(&mut y_or_n).unwrap();
            if y_or_n == "n" {
                exit(1);
            }
        }
        let mut file =
            File::create(parsed_args.file).expect("Error encountered while creating file!");
        for mut line in full_repl {
            if line.eq("exit") {
                break;
            }
            line.push('\n');
            file.write_all(line.as_bytes())
                .expect("Error encountered while writing to file!");
        }
    }
}

// turn the above into a function
