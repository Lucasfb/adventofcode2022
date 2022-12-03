fn main() {
    println!("Advent of Code 2022 - Day 2");

    let input = open_input_file();

    let mut total_score = 0;
    let list_matches = input.lines();
    for strategy in list_matches {
        let mut turn: String = strategy.split(" ").collect();
        let my_move = turn.pop().unwrap();
        let opponent_move = turn.pop().unwrap();
        total_score = total_score + match_score_part1(&opponent_move,&my_move);
    }
    println!("Answer for Part One: {}",total_score);

    // Start of part two
    let mut total_score = 0;
    let list_matches = input.lines();
    for strategy in list_matches {
        let mut turn: String = strategy.split(" ").collect();
        let my_move = turn.pop().unwrap();
        let opponent_move = turn.pop().unwrap();
        total_score = total_score + match_score_part2(&opponent_move,&my_move);
    }
    println!("Answer for Part Two: {}",total_score);
}



fn match_score_part1(opponent_move: &char, my_move: &char) -> u32 {
    match (opponent_move,my_move) {
        ('A','X') => 3+1,
        ('A','Y') => 6+2,
        ('A','Z') => 0+3,
        ('B','X') => 0+1,
        ('B','Y') => 3+2,
        ('B','Z') => 6+3,
        ('C','X') => 6+1,
        ('C','Y') => 0+2,
        ('C','Z') => 3+3,
        _ => panic!()
    }
}

fn match_score_part2(opponent_move: &char, my_move: &char) -> u32 {
    match (opponent_move,my_move) {
        ('A','X') => 3+0,
        ('A','Y') => 1+3,
        ('A','Z') => 2+6,
        ('B','X') => 1+0,
        ('B','Y') => 2+3,
        ('B','Z') => 3+6,
        ('C','X') => 2+0,
        ('C','Y') => 3+3,
        ('C','Z') => 1+6,
        _ => panic!()
    }
}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}

