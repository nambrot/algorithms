extern crate rand;
extern crate rayon;
mod mincut;
use mincut::Graph;

fn main() {
    let g: Graph = mincut::parse_file("../raws/mincut.txt");
    println!("{}", mincut::karger(&g));;
}
