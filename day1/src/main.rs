use std::env;
use std::fs;
use std::cmp::Ordering;

#[derive(Eq)]
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

    let input_filename = "aoc2022_day1a_input.txt";
    let input = fs::read_to_string(input_filename).expect("Could not open file");

    let elf_list_string:Vec<&str> = input.split("\n\n").collect(); // Split on double newline (empty lines)
//    println!("{}",elf_list[0]);

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
        println!("{}",max.calories_total)
        // Answer for part one of the puzzle
    }
}
