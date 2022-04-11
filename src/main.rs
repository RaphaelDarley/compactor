use compactor::*;

fn main() {
    // println!("Hello, world!");

    // let test = huffman::list_to_tree(huffman::str_to_tokfreq(&"affaaaffddaaaddfdgdgddgffffddddd").unwrap());

    // let test = huffman::str_to_tokfreq(&"affaaaffddaaaddfdgdgddgffffddddd").unwrap();

    // let test_tree = huffman::list_to_tree(
    //     huffman::str_to_tokfreq(&"affaaaffddaaaddfdgdgddgffffddddd").unwrap(),
    // );

    // let test = huffman::tree_to_code(test_tree, vec![]);

    let (test_encode, test_tree) = huffman::encode(&"this is a test message");

    println!("encoding: {:?}", test_encode);
    println!("encoding: {:?}", test_tree);

    let test_decode = huffman::decode(test_encode, &test_tree);

    println!("decoded: {:?}", test_decode);
}
