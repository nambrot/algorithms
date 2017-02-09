extern crate rand;
use rand::Rng;
mod karatsuba;

fn main() {
    let x : usize = rand::thread_rng().gen_range(1, 101000000000);
    let y : usize = rand::thread_rng().gen_range(1, 101000000000);

    println!("The product of");
    println!("x = {}", x);
    println!("y = {}", y);
    println!("is x*y    = {}", x * y);
    println!("karatsuba = {}", karatsuba::mult(x, y))
}
