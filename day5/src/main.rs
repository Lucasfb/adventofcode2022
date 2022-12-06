/*
Each stack is a collection of ordered elements
Stacks can change sizes
Stack can be empty
Stack doesn't have a specific maximum size
An element is added to the end of the stack
An element is removed from the end of the stack

There are multiple of stacks
The number of stacks is fixed
Each stack is identified

Data Structure Representations
* Stack -> Vec<char>
* List of Stacks -> Array
*/ 
    

/*
# How to parse the input string into a data structure of the crate stacks
* Split the input (crate stack section only) into lines
* Find the line that contains the identifiers for each stack (last line)
* Find the index of the string at which each identifier is
* For the other lines, if the element at each index found is a char, then that is the crate. If the char is ' ', then there is no crate at that position
*/
use std::{str::FromStr, collections::VecDeque};

#[derive(Debug, Clone, Copy)]
struct MoveOrder {
    qty: usize,
    origin: usize,
    destination: usize,
}

impl FromStr for MoveOrder {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let move_order_split: Vec<&str> = s.split_whitespace().collect();
        let new_move_order = MoveOrder {
            qty: move_order_split[1].parse::<usize>().unwrap(),
            origin: move_order_split[3].parse::<usize>().unwrap(),
            destination: move_order_split[5].parse::<usize>().unwrap(),
        };
        Ok(new_move_order)
    }
}

fn main() { 
    println!("Advent of Code 2022 - Day 3");
    let input = open_input_file();


    let (starting_input,movement_input) = input.split_once("\n\n").expect("Empty separating line not found");

    let mut stack_list = create_stack_list(starting_input);

    let move_orders_list:Vec<MoveOrder> = movement_input.lines()
                                        .map(|order| order.parse::<MoveOrder>()
                                                .expect("Could not parse the move"))
                                        .collect();
    //println!("{:?}",move_orders_list[25]);

    for order in move_orders_list {
        for _ in 0..order.qty {
            let crate_moved = stack_list[order.origin-1].pop().unwrap();
            stack_list[(order.destination)-1].push(crate_moved); 
        }
    }

    let top_of_stacks: Vec<&char> = stack_list.iter()
                                            .map(|i|i
                                                            .last()
                                                            .unwrap())
                                            .collect();
    println!("Answer of Part One: {:?}",top_of_stacks);

    // Start of Part Two

    let (starting_input,movement_input) = input.split_once("\n\n").expect("Empty separating line not found");

    let mut stack_list = create_stack_list(starting_input);

    let move_orders_list:Vec<MoveOrder> = movement_input.lines()
                                        .map(|order| order.parse::<MoveOrder>()
                                                .expect("Could not parse the move"))
                                        .collect();

    for order in move_orders_list {
        let mut crates_moved:VecDeque<char> = VecDeque::new();
        for _ in 0..order.qty {
            crates_moved.push_front(stack_list[order.origin-1].pop().unwrap());
            
        }
        for _ in 0..order.qty {
            stack_list[(order.destination)-1].push(crates_moved.pop_front().unwrap()); 
        }
    }

    let top_of_stacks: Vec<&char> = stack_list.iter()
                                            .map(|i|i
                                                            .last()
                                                            .unwrap())
                                            .collect();
    println!("Answer of Part Two: {:?}",top_of_stacks);
}

fn create_stack_list(input: &str) -> [Vec<char>;9] {
    let mut stack_list :[Vec<char>;9] = Default::default();

    let mut input_lines: Vec<&str> = input.lines().collect();
    let identifier_line = input_lines.pop().expect("Input should not be empty");
    let identifiers: Vec<char> = identifier_line.split_whitespace().collect::<String>().chars().collect();   
    let identifiers_index: Vec<usize> = identifiers.into_iter().map(|i| identifier_line.find(i).unwrap()).collect();
    
    //println!("{:?}",identifiers_index);
    let input_lines: Vec<&str> = input_lines.into_iter().rev().collect();
    for line in input_lines {
        for idx in 0..identifiers_index.len() {
            let package: char =line
                                .chars()
                                .nth(identifiers_index[idx])
                                .expect("Not a char"); 
            if package.is_whitespace() {
                // skip
            } else {
            stack_list[idx].push(package)
            }
        }
    }

    //println!("{:?}",stack_list);
    return stack_list
}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}