use std::env;
use std::fs::File;

use std::io::{self, prelude::*, BufReader};

fn main() {
    let makefile: String;
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let out: String = "a.out".to_string();
    if args.len() >= 2 {

        makefile = match args[1] {

            // Otherwise, check if the 1st argument is ends with .cmp
            _ if args[1].ends_with("cmpfile") => {
                args[1].to_string()
            },
            _ => {
                eprintln!("Usage: compiler <cmpfile>");
                std::process::exit(1);
            }
        };
    } else {
        println!("repl");
        std::process::exit(0); }

    // oepn the file with reading permissions
    let mut file = File::open(makefile).expect("File not found");

    // create a new file
    println!("{}", out);
    let mut outfile = File::create(out).expect("Could not create file");
    let reader = BufReader::new(file);
    let mut src = reader.lines();
    let mut codefile = src.nth(0).unwrap().expect("Could not open file");
    codefile = codefile.split("= ").nth(1).unwrap().to_string();
    println!("{}", codefile);
    // println!("{}", );
    let mut code = File::open(codefile).expect("Could not open file");

    // print the file contents
    let mut contents = String::new();
    code.read_to_string(&mut contents).expect("Could not read file");
    // write the contents to the new file
    outfile.write_all(contents.as_bytes()).expect("Could not write to file");
    
    // println!("{}", contents);

    // close the file
    }
