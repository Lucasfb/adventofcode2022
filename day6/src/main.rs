fn is_start_of_packet_marker(characters_4: &str) -> Result<bool,&str> {
    if characters_4.len() != 4 {
        return  Err("Possible start of packet marker should have a lenght of 4 characters");
    }
    let mut letters = characters_4.chars();
    Ok(different_4_chars(letters.next().unwrap(),
                        letters.next().unwrap(),
                        letters.next().unwrap(),
                        letters.next().unwrap()))
}

fn different_4_chars(a:char,b:char,c:char,d:char,) -> bool {
    a != b && a != c && a != d && b != c && b != d && c != d 
}

fn main() {
    println!("Advent of Code 2022 - Day 6");
    let input = open_input_file();

    //let distinct_characters_count = 0;
    let mut start_of_packet_found = false;
    
    let mut start_of_packet_index = 0;
    let mut end_of_packet_index = start_of_packet_index + 3;

    while !start_of_packet_found {
        let try_input = input.get(start_of_packet_index..=end_of_packet_index).unwrap();
        if is_start_of_packet_marker(try_input).unwrap() {
            start_of_packet_found = true;
        } else {
            start_of_packet_index += 1;
            end_of_packet_index +=1; 
        }
        
    }
    println!("Answer of Part One: {:?}",end_of_packet_index+1);

}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}