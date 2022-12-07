use itertools::Itertools;

fn find_position_end_of_marker(input: &str,size_of_window: usize) -> Option<usize> {
    input
        .as_bytes()  //Hard to work with chars, so it's easier to use u8
        .windows(size_of_window)
        .position(|window| window
                                        .iter()
                                        .unique()
                                        .count() == size_of_window)
        .map(|pos|pos+size_of_window)
}

fn main() {
    println!("Advent of Code 2022 - Day 6");
    let input = open_input_file();


    println!("Answer of Part One: {:?}",find_position_end_of_marker(&input, 4).unwrap());

    // Start of Part Two

    println!("Answer of Part One: {:?}",find_position_end_of_marker(&input, 14).unwrap());
  
}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}