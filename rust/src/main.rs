extern crate rand;
extern crate rayon;
extern crate pbr;
use rand::Rng;
mod quickselect;
mod prim;
fn main() {
    let arr = prim::parse_file("../raws/primmst.txt");
    println!("{:?}", prim::minimum_spanning_tree(&arr));
}
