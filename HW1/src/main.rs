//!Homework 1 for CS410P - Rust
// Ian Guy 2022

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

    if numbers.len() < 3 {
        error();
    }
    
    let mut x = parsenum(numbers[0]);
    let mut y = parsenum(numbers[1]);
    let mut m = parsenum(numbers[2]);

    if (m == 0) || (u128::from((m-1)*(m-1)) == u128::MAX) {
        error();
    }
}