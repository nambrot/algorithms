extern crate rand;
extern crate rayon;
extern crate pbr;
use rand::Rng;
mod quickselect;
mod jobs;
fn main() {
    let mut arr = jobs::parse_file("../raws/greedyjobs.txt");
    println!("{:?}", jobs::weighted_schedule_completion(&jobs::schedule_jobs(&arr)));
}
