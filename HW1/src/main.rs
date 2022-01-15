//!Homework 1 for CS410P - Rust
// Ian Guy 2022

use std::env;

struct Arguments {
    x: u64,
    y: u64,
    m: u64,
}

//Error tool from hints
fn error() -> ! {
    eprintln!("modexp: useage: modexp <x> <y> <m>");
    std::process::exit(1);
}

//Parse tool from hints
fn parsenum(s: String) -> u64 {
    s.parse().unwrap_or_else(|_| error())
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 3 {
        error();
    }

    Arguments {
        x: parsenum(args[0].clone()),
        y: parsenum(args[1].clone()),
        m: parsenum(args[2].clone()),
    }
}

//modexp function which does the actual math. if doing unit testing, make it return a u64!
fn modexp(x: u64, mut y: u64, m: u64) {
    let mut ans = 1;
    let mut z = u128::from(x);
    if m == 0 {
        error();
    } else if m == 1 {
        return;
    }

    while y > 0 {
        if y % 2 == 1 {
            ans = (ans * z) % u128::from(m);
        }
        y /= 2;
        z = (z * z) % u128::from(m);
    }
    println!("{}", ans);
    //below return statement is needed for tests to see
    //return ans.try_into().unwrap();
}

fn main() {
    let args = parse_args();
    modexp(args.x, args.y, args.m);
}

//FROM THE HOMEWORK
#[test]
fn test_modexp() {
    // Largest prime less than 2**64.
    // https://primes.utm.edu/lists/2small/0bit.html
    //let bigm = u64::max_value() - 58;
    //assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
    //assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
    //assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
    // https://practice.geeksforgeeks.org/problems/
    //    modular-exponentiation-for-large-numbers/0
    //assert_eq!(4, modexp(10, 9, 6));
    //assert_eq!(34, modexp(450, 768, 517));
}
