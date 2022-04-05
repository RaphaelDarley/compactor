use compactor::*;

fn main() {
    println!("Hello, world!");

    // let test = huffman::list_to_tree(huffman::str_to_tokfreq(&"affaaaffddaaaddfdgdgddgffffddddd").unwrap());

    // let test = huffman::str_to_tokfreq(&"affaaaffddaaaddfdgdgddgffffddddd").unwrap();

    let test_tree = huffman::list_to_tree(
        huffman::str_to_tokfreq(&"affaaaffddaaaddfdgdgddgffffddddd").unwrap(),
    );

    let test = huffman::tree_to_code(test_tree, vec![]);

    println!("{:?}", test);
}
