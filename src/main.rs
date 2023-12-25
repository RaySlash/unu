// Wrapper to nix and pacman
// -- Priorities
// pacman = 1000
// aur = 999
// nix = 998
//
// Basic working Principles
// From user (Vec<strings>) as arguments
// Parse and dump in terminal with pacman/nix acc to priority
//
// Forcasted commands:  ( ?? indicate optional )
// unu function package
// eg: unu install cargo
use std::{process, env};

struct Config {
    function: String,
    package: String,
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    
    let config = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments! {err}");
        process::exit(1);
    });

    let (_x,_y,z) = ("pacman", "aur", "nix");

    let output = process::Command::new(format!("{}", z)) // Set x/y/z
        .arg(config.function)
        .arg(config.package)
        .output().unwrap_or_else(|e| {
            panic!("Failed to execute process: {}", e)
    });

    if output.status.success() {
        let tmp = String::from_utf8_lossy(&output.stdout);
        println!("Success: {}", tmp);
    } else {
        let tmp = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", tmp);
    }

}

fn parse_args(args: &[String]) -> Result<Config, &'static str> {

    if args.len() != 2 {
        return Err("Program expects 2 arguments!");
    } else {
        let function= &args[1];
        let package = &args[2];

        Ok(Config {function: function.to_string(), package: package.to_string(), })
    }
}
