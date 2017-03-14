extern crate rand;
extern crate rayon;
extern crate pbr;

mod twosum;

fn main() {
    let numbers = twosum::parse_file("../raws/twosum.txt");
    println!("{:?}", twosum::number_of_target_values_in(&numbers, -10000, 10000));
}
