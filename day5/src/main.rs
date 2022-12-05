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

*/ 
    




fn main() { 
    println!("Advent of Code 2022 - Day 3");
    let input = open_input_file();

    let stack_list :[Vec<char>;9] = Default::default();

}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}