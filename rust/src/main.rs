extern crate rand;
use rand::Rng;
use std::io::prelude::*;
use std::fs::File;
mod inversions;


fn main() {
    let mut s = String::new();
    let mut file = File::open("../raws/num_of_inversions_in_array.txt").unwrap();
    file.read_to_string(&mut s).unwrap();

    let arr: Vec<u32> =
        s.split_whitespace()
            .map(|string: &str| string.parse::<u32>().unwrap())
            .collect();
    println!("{:?}", arr);
    println!("{:?}", inversions::count(&arr));
}
