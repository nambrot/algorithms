extern crate rand;
extern crate rayon;
extern crate pbr;

mod median_maintenance;

fn main() {
    let numbers = median_maintenance::parse_file("../raws/medians.txt");
    println!("{:?}", median_maintenance::median(&numbers));
}
