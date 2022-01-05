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
        x = (x * x) % m;
    }
    println!("{}", ans);
}

//FROM THE HOMEWORK
#[test]
fn test_modexp() {
    // Largest prime less than 2**64.
    // https://primes.utm.edu/lists/2small/0bit.html
    let bigm = u64::max_value() - 58;
    assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
    assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
    assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
    // https://practice.geeksforgeeks.org/problems/
    //    modular-exponentiation-for-large-numbers/0
    assert_eq!(4, modexp(10, 9, 6));
    assert_eq!(34, modexp(450, 768, 517));
}