fn main() {
    println!("Advent of Code 2022 - Day 6");
    let input = open_input_file();

    let distinct_characters_count = 0;

    
}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}