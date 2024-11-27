use std::env;
use json_pest_parser::parse_json_str;
use json_pest_parser::json_content_to_Str;
use std::fs;
use std::process;

fn print_help() {
    println!("Hi! This is the program which parses .json files using pest library. Brief intro how to use program:");   
    println!("cargo run -- example.json  => To parse entered .json file amd get result.");
    println!("cargo run -- --help        => To see instruction again.");
    println!("cargo run -- --credits     => To show credits.");
}

fn print_creds()
{
    println!("JSON file format parser");
    println!("Author: Solovei Tymofii");
    println!("GitHub: @amitlos");
    println!("Version 1.0");

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_help();
        process::exit(1);
    }

    match args[1].as_str() {
        "--help" => {
            print_help();
        }
        "--credits" => {
            print_creds();
        }
        _ => {
            // Handle file path
            let file_path = &args[1];


            let file = fs::read_to_string(file_path).expect("cannot read file");

            match parse_json_str(&file)
            {
                Ok(json_conent) => println!("{}", json_content_to_Str(&json_conent)),
                Err(err) => println!("Error occured: {}", err)
            }
        }
    }
}