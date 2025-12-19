use std::fs;
pub fn get_input_instructions(file_location: &str) -> Vec<String> {
    let file_content: String = fs::read_to_string(file_location).expect("I CAN READ THIS RIGHT??");

    let split_vec: Vec<String> = file_content.split("\n").map(|s| s.to_string()).collect();
    return split_vec;
}
