extern crate rand;
use rand::Rng;
mod mergesort;


fn main() {
    let arr : Vec<u32> = vec![0; rand::thread_rng().gen_range(1, 10)]
                .into_iter()
                .map(|_x| rand::thread_rng().gen_range(1, 100))
                .collect();

    println!("{:?}", arr);
    println!("{:?}", mergesort::sort(&arr));

}
