mod swappie;
mod flags;

use swappie::Swappie;
use flags::parse_args;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let flags = match parse_args(args[1..].to_vec()) {
        Ok(f) => f,
        Err(s) => {
            println!("[ERROR] {}", s);
            process::exit(1);
        }
    };

    if flags.text.is_empty() {
        println!("[ERROR] No text was given as input");
        process::exit(1);
    }

    let swappie = Swappie::new();
    match swappie.mirror_text(flags.text.clone(), flags.allow_mangle) {
        Ok(s) => {
            if s.1 && flags.verbose {
                println!(
                    "[WARNING] Could not mirror it perfectly:\nOriginal  ->  {}\nMirrored  ->  {}",
                    flags.text, s.0
                );
            } else if !s.1 && flags.verbose {
                println!(
                    "[SUCCESS] Successfully mirrored the text:\nOriginal  ->  {}\nMirrored  ->  {}",
                    flags.text, s.0
                );
            } else {
                println!("{}", s.0);
            }
        }
        Err(s) => {
            println!("[ERROR] {}", s);
            process::exit(1);
        }
    }
}
