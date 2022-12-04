


fn main() {
    println!("Advent of Code 2022 - Day 3");
    let input = open_input_file();
    
    let mut sum_priorities = 0;


    for backpack in input.lines() {
        let (left_compartment,right_compartment) = backpack.split_at(backpack.len()/2);
        for item in left_compartment.chars() {
            if  right_compartment.contains(item) {
                sum_priorities += priority_value(item);
                break;
            }
        }
    }
    println!("Answer for Part One: {}",sum_priorities);


    // Start of Part Two
    let mut backpacks_list: Vec<&str> = input.lines().collect();
    let mut sum_priorities = 0;
    
    while backpacks_list.is_empty() != true {
        // Since order doesn't matter, only the sum of priorities, we can start from the bottom
        let (first_backpack, second_backpack, third_backpack) = (backpacks_list.pop().unwrap(),backpacks_list.pop().unwrap(),backpacks_list.pop().unwrap());
        
        for item in first_backpack.chars() {
            if second_backpack.contains(item) & third_backpack.contains(item) {
                sum_priorities += priority_value(item);
                break;
            }
        }
    }
    println!("Answer for Part One: {}",sum_priorities)

   
    
}

fn priority_value(item: char) -> usize {

    match item {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("Not a reconized value")
    }
}


fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}

