use compactor::*;

fn main() {
    println!("Hello, world!");

    let test = huffman::list_to_tree(huffman::str_to_tokfreq(&"affaaaffddaaaddfdgdgddgffffddddd").unwrap());

    // let test = huffman::str_to_tokfreq(&"affaaaffddaaaddfdgdgddgffffddddd").unwrap();

    println!("{:?}", test);
}
