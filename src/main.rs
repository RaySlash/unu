// Wrapper to nix and pacman
// -- Priorities
// pacman = 1000
// aur = 999
// nix = 998
// ........
//
// Basic working Principles
// From user (Vec<strings>) as arguments
// Parse and dump in terminal with pacman/nix acc to priority
//
// Forcasted commands:  ( ?? indicate optional )
// unu package ?{source}? 
// eg: unu cargo nix
use std::{process, env};
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    package: String,
    source: String,
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    
    let config = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments! {err}");
        process::exit(1);
    });

    println!("{}", config.package);
    println!("{}", config.source);

}

fn parse_args(args: &[String]) -> Result<Config, &'static str> {
    if !args.len() == 3 {
        return Err("Program expects 2 arguments!");
    } else {
        let package = &args[1];
        let source = &args[2];

        Ok(Config { package: package.to_string(), source: source.to_string() })
    }
}
