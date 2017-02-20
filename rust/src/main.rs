extern crate rand;
use rand::Rng;
use std::io::prelude::*;
use std::fs::File;
mod quicksort;


fn main() {
    let mut s = String::new();
    let mut file = File::open("../raws/quicksort.txt").unwrap();
    file.read_to_string(&mut s).unwrap();

    let mut arr: Vec<u32> = s.split_whitespace()
        .map(|string: &str| string.parse::<u32>().unwrap())
        .collect();

    println!("{:?}", arr);
    quicksort::sort(&mut arr);
    println!("{:?}", arr);
}
