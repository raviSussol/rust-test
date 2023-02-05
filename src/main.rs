use rust_test::{
    add::add,
    read_file::{read_file_as_buffer_string, read_file_as_string},
    subtract::subtract,
};

fn main() {
    println!("Addition: {}", add(4, 5));
    println!("Substraction: {}", subtract(10, 3));
    let file_path = "test_file.txt";

    // Read file through buffer reader
    let contents1 = read_file_as_buffer_string(file_path).unwrap();
    println!("{}", contents1);

    // Read file through file string
    let content2 = read_file_as_string(file_path).unwrap();
    println!("{}", content2);
}
