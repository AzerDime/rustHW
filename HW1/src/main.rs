//!Homework 1 for CS410P - Rust
// Ian Guy 2022

use std::io;

//Error tool from hints
fn error() -> !{
    eprintln!("modexp: useage: modexp <x> <y> <m>");
    std::process::exit(1);
}

//Parse tool from hints
fn parsenum(s: &str) -> u64 {
    s.parse().unwrap_or_else(|_| error())
}

fn main(){
    //using some code from the rust-cli github page for command line inputs
    let args = Cli::from_args();
    
}