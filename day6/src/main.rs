use itertools::Itertools;
/* 
fn all_different_chars(input: &str) -> bool {
    let initial_size = input.len();
    let mut dedup_input = input.chars().map(|c| c as u8).collect::<Vec<u8>>();
    dedup_input.sort(); // Doesn't seem to scale well
    dedup_input.dedup();
    return initial_size == dedup_input.len()
} */

fn find_position_end_of_marker(input: &str,size_of_window: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(size_of_window)
        .position(|window| window
                                        .iter()
                                        .unique()
                                        .count() == size_of_window)
        .map(|pos|pos+size_of_window)

    //None
}

fn main() {
    println!("Advent of Code 2022 - Day 6");
    let input = open_input_file();

    //let distinct_characters_count = 0;
 /*    let mut start_of_packet_found = false;
    
    let mut start_of_packet_index = 0;
    let mut end_of_packet_index = start_of_packet_index + 3;

    while !start_of_packet_found {   

        let try_input = input.get(start_of_packet_index..=end_of_packet_index).unwrap();
        if all_different_chars(try_input) {
            start_of_packet_found = true;
        } else {
            start_of_packet_index += 1;
            end_of_packet_index +=1; 
        }
        
    } */
    println!("Answer of Part One: {:?}",find_position_end_of_marker(&input, 4).unwrap());
    //println!("Answer of Part One: {:?}",end_of_packet_index+1);

    // Start of Part Two

   /*  let mut start_of_message_found = false;
    
    let mut start_of_message_index = 0;
    let mut end_of_message_index = start_of_message_index + 13;

    while !start_of_message_found {

        let try_input = input.get(start_of_message_index..=end_of_message_index).unwrap();
        if all_different_chars(try_input) {
            start_of_message_found = true;
        } else {
            start_of_message_index += 1;
            end_of_message_index +=1; 
        }
        
    } */
    println!("Answer of Part One: {:?}",find_position_end_of_marker(&input, 14).unwrap());
    //println!("Answer of Part One: {:?}",end_of_message_index+1);

    
}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}