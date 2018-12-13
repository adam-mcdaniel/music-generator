#![allow(unused_imports)]
use std::env;
use std::process;

extern crate music_generator;

use music_generator::chord;
use music_generator::song::*;
use music_generator::song::Key::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: music_generator KEY VERSES CHORUSES");
        process::exit(0);
    }

    println!("Generating music");
    println!("{}",
        Song::new(
            match args[1].as_str() {
                "A" => A,
                "B" => B,
                "C" => C,
                "D" => D,
                "E" => E,
                "F" => F,
                "G" => G,
                _ => C
            },
            args[3].parse::<i32>().unwrap(),
            args[2].parse::<i32>().unwrap()
        )
    );
}