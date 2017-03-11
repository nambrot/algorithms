extern crate rand;
extern crate rayon;
extern crate pbr;

mod scc;
use scc::Graph;

fn main() {
    let g: Graph = scc::parse_file("../raws/scc.txt");
    println!("{:?}", scc::strongest_connected_components(&g, 10));
}
