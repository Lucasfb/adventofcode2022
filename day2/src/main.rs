fn main() {
    println!("Advent of Code 2022 - Day 2");

    let input = open_input_file();

    let mut total_score = 0;
    let list_matches = input.lines();
    for strategy in list_matches {
        let mut turn: String = strategy.split(" ").collect();
        let my_move = turn.pop().unwrap();
        let opponent_move = turn.pop().unwrap();
        total_score = total_score + match_score(&opponent_move,&my_move);
    }

    println!("Answer for Part One: {}",total_score)

}



fn match_score(opponent_move: &char, my_move: &char) -> u32 {
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


fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}

