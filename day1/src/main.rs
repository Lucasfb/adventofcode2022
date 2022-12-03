use std::cmp::Ordering;

#[derive(Eq,Clone)]
struct Elf {
    food_stock : Vec<u32>,
    calories_total: u32,
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.calories_total.cmp(&other.calories_total)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calories_total == other.calories_total
    }
}

fn main() {
    println!("Advent of Code 2022 - Day 1");

    let input = open_input_file();
    let elf_list_string:Vec<&str> = input.split("\n\n").collect(); // Split on double newline (empty lines)

    let mut elf_list : Vec<Elf> = Vec::new();
    for elf_string in elf_list_string {
        let food_list_str:Vec<&str> = elf_string.lines().collect(); 
        let food_list:Vec<u32> = food_list_str.iter().map(|&x| x.parse::<u32>().expect("Not a number")).collect();
        let new_elf: Elf  = Elf {
            food_stock: food_list.clone(),
            calories_total : food_list.iter().sum(),
        };
        elf_list.push(new_elf);
    }

    let most_calories_in_elf = elf_list.iter().max();
    if let Some(max) = most_calories_in_elf {
        println!("Answer for Part One: {}",max.calories_total)
        // Answer for part one of the puzzle
    }

    // Start of part two of puzzle
    // Sort the list by calories_total and pick the top three (last elements)
    elf_list.sort();
    let elf_first_most_calories = &elf_list[elf_list.len()-1];
    let elf_second_most_calories = &elf_list[elf_list.len()-2];
    let elf_third_most_calories = &elf_list[elf_list.len()-3];

    let total_top3_calories = &elf_first_most_calories.calories_total +
                                &elf_second_most_calories.calories_total + 
                                &elf_third_most_calories.calories_total;
    println!("Answer for Part Two: {}",total_top3_calories); // Answer for part two of the puzzle


}

fn open_input_file() -> String {
    let input_filename = "aoc2022_day1a_input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}