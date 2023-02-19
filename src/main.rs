use rust_test::{
    math::Math,
    read_file::{read_file_as_buffer_string, read_file_as_string},
};

fn main() {
    println!("Addition: {}", Math::add(4, 5));
    println!("Substraction: {}", Math::subtract(10, 3));
    let file_path = "test_file.txt";

    // Read file through buffer reader
    let contents1 = read_file_as_buffer_string(file_path).unwrap();
    println!("{}", contents1);

    // Read file through file string
    let content2 = read_file_as_string(file_path).unwrap();
    println!("{}", content2);
}
