


fn main() {
    println!("Advent of Code 2022 - Day 3");
    let input = open_input_file();
    
    let mut sum_priorities = 0;
    let mut count_adds = 0;

    for backpack in input.lines() {
        let (left_compartment,right_compartment) = backpack.split_at(backpack.len()/2);
        //println!("Left: {}, len: {}, Right: {}, len {}",left_compartment,left_compartment.len(),right_compartment,right_compartment.len());
        for item in left_compartment.chars() {
            if  right_compartment.contains(item) {
                //println!("Left: {}, Right: {}, Char {} - Value {}",left_compartment,right_compartment,item,priority_value(item));
                //println!("Char {} - Value {}",item,priority_value(item));
                sum_priorities += priority_value(item);
                count_adds += 1;
                break;
            }
        }
    }

    println!("Count of additions: {}",count_adds);
    println!("Answer for Part One: {}",sum_priorities);
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

