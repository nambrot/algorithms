extern crate rand;
extern crate rayon;
extern crate pbr;
use rand::Rng;
mod quickselect;
mod median_maintenance;
fn main() {
    let mut arr = median_maintenance::parse_file("../raws/num_of_inversions_in_array.txt");
    let k = rand::thread_rng().gen_range(1, arr.len());
    println!("{}", quickselect::select(&arr, k));
    arr.sort();
    println!("{}", arr[k-1]);
}
