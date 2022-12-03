fn main() {
    println!("Advent of Code 2022 - Day 1");

    let input = open_input_file();
    let elf_list_string:Vec<&str> = input.split("\n\n").collect(); // Split on double newline (empty lines)

    let mut calorie_sum_list : Vec<u32> = Vec::new();
    for elf_string in elf_list_string {
        let food_list_str:Vec<&str> = elf_string.lines().collect(); 
        let food_list:Vec<u32> = food_list_str.iter().map(|&x| x.parse::<u32>().expect("Not a number")).collect();
        let calories_total = food_list.iter().sum();
        calorie_sum_list.push(calories_total);
    }
        
    let most_calories_in_elf = calorie_sum_list.iter().max();
    if let Some(max) = most_calories_in_elf {
        println!("Answer for Part One: {}",max)
        // Answer for part one of the puzzle
    }

    // Start of part two of puzzle
    // Sort the list by calories_total and pick the top three (last elements)
    calorie_sum_list.sort();

    let elf_first_most_calories = calorie_sum_list[calorie_sum_list.len()-1];
    let elf_second_most_calories = calorie_sum_list[calorie_sum_list.len()-2];
    let elf_third_most_calories = calorie_sum_list[calorie_sum_list.len()-3];

    let total_top3_calories = elf_first_most_calories +
                                elf_second_most_calories + 
                                elf_third_most_calories;
    println!("Answer for Part Two: {}",total_top3_calories); // Answer for part two of the puzzle


}

fn open_input_file() -> String {
    let input_filename = "aoc2022_day1a_input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}