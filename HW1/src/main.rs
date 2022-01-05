//!Homework 1 for CS410P - Rust
// Ian Guy 2022

use std::env;

//Error tool from hints
fn error() -> ! {
    eprintln!("modexp: useage: modexp <x> <y> <m>");
    std::process::exit(1);
}

//Parse tool from hints
fn parsenum(s: &str) -> u64 {
    s.parse().unwrap_or_else(|_| error())
}

fn main() {
    //let numbers = Vec::new();
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 3 {
        error();
    }
    println!("Correct value amount");
    let mut x = parsenum(&args[0]);
    let mut y = parsenum(&args[1]);
    let m = parsenum(&args[2]);
    let mut ans = 1;
    if (m == 0) || (u128::from((m - 1) * (m - 1)) == u128::MAX) {
        error();
    } else if m == 1 {
        return;
    }

    while y > 0 {
        if y % 2 == 1 {
            ans = (ans * x) % m;
        }
        y /= 2;
        x = (x ^ 2) % m;
    }
    println!("{}", ans);
}
