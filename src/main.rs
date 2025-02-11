fn main() {
    let input_file_name = "input.txt";
    let contents = std::fs::read_to_string(input_file_name).expect("Failed to read the input file");
    println!("{}", contents.len());
}
