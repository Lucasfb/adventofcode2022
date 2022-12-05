use std::ops::{RangeBounds, RangeInclusive};

fn main() {
    println!("Advent of Code 2022 - Day 3");
    let input = open_input_file();

    let mut pair_fully_contains_count = 0;
    for pair in input.lines() {
        let ranges: Vec<&str> = pair.split(',').collect();
        let first_range : Vec<&str> = ranges.get(0).unwrap().split('-').collect::<Vec<&str>>();
        let second_range : Vec<&str> = ranges.get(1).unwrap().split('-').collect::<Vec<&str>>();
        let (first_range_start,first_range_end) = (first_range.get(0).unwrap().parse::<usize>().unwrap(),
                                                            first_range.get(1).unwrap().parse::<usize>().unwrap());
        let (second_range_start,second_range_end) = (second_range.get(0).unwrap().parse::<usize>().unwrap(),
                                                            second_range.get(1).unwrap().parse::<usize>().unwrap());
        let first_range = first_range_start..=first_range_end;
        let second_range = second_range_start..=second_range_end;

        if ranges_fully_overlap(first_range,second_range)  {
            pair_fully_contains_count += 1;
        }
       
    }
    println!("Answer for Part One: {}",pair_fully_contains_count);

    // Start of Part Two

    let mut pair_any_overlap_count = 0;
    for pair in input.lines() {
        let ranges: Vec<&str> = pair.split(',').collect();
        let first_range : Vec<&str> = ranges.get(0).unwrap().split('-').collect::<Vec<&str>>();
        let second_range : Vec<&str> = ranges.get(1).unwrap().split('-').collect::<Vec<&str>>();
        let (first_range_start,first_range_end) = (first_range.get(0).unwrap().parse::<usize>().unwrap(),
                                                            first_range.get(1).unwrap().parse::<usize>().unwrap());
        let (second_range_start,second_range_end) = (second_range.get(0).unwrap().parse::<usize>().unwrap(),
                                                            second_range.get(1).unwrap().parse::<usize>().unwrap());
        let first_range = first_range_start..=first_range_end;
        let second_range = second_range_start..=second_range_end;

        if ranges_any_overlap(first_range,second_range)  {
            pair_any_overlap_count += 1;
        }
       
    }
    println!("Answer for Part Two: {}",pair_any_overlap_count);
}

fn ranges_fully_overlap(first: RangeInclusive<usize>,second: RangeInclusive<usize>) -> bool{
    (first.contains(second.start()) && first.contains(second.end()))|| 
    (second.contains(first.start()) && second.contains(first.end()))

}

fn ranges_any_overlap(first: RangeInclusive<usize>,second: RangeInclusive<usize>) -> bool{
    (first.contains(second.start()) || first.contains(second.end()))|| 
    (second.contains(first.start()) || second.contains(first.end()))

}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}

