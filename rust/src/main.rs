extern crate rand;
extern crate rayon;
extern crate pbr;
mod huffman;

fn main() {
    let arr = huffman::HuffmanCode::from_sample_text("Im really good text");
    println!("{:?}", arr);
    let string = "Im really good text".to_string();
    println!("Encode {} to {} and then decode again {}", string, arr.encode(&string), arr.decode(&arr.encode(&string)));
}
