//!Homework 1 for CS410P - Rust
// Ian Guy 2022

use std::str::FromStr;
use std::env;

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
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.parsenum(u64::from_str(&arg));
    }

    if numbers.len() == 0 {
        error();
    }
    
    let mut x = numbers[0];
    let mut y = numbers[1];
    let mut m = numbers[2];

    if (m <= 0) || (m >= u128::MAX) {
        error();
    }
}