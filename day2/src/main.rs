use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, other:Move) -> bool {
        matches!((self,other),
            (Self::Rock,Self::Scissors)
            | (Self::Paper,Self::Rock)
            | (Self::Scissors,Self::Paper)
        )
    }

    fn outcome(self,theirs:Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss 
        } else {
            Outcome::Draw
        }
    }
}

impl TryFrom<char> for Move {
    type Error = &'static str;

    fn try_from(c:char) -> Result<Self,Self::Error> {
        match c {
            'A'|'X' => Ok(Move::Rock),
            'B'|'Y' => Ok(Move::Paper),
            'C'|'Z' => Ok(Move::Scissors),
            _ => Err("Invalid move")
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }

    fn our_score(self) -> usize {
        self.ours.inherent_points() + self.outcome().inherent_points()
    }
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours),None) = 
        (chars.next(),chars.next(),chars.next(),chars.next()) 
        else {
            return Err("Line does not match pattern <theirs>SP<ours>EOF")
        };

        Ok(Self{
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }

}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn inherent_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

fn main() {
    println!("Advent of Code 2022 - Day 2");

    let input = open_input_file();

    let mut total_score = 0;
    let list_matches = input.lines();
    for strategy in list_matches {
        let round = strategy.parse::<Round>().unwrap(); 
        let round_score = round.our_score();
        total_score += round_score;
        /*
        let mut turn: Round = strategy.split(' ').collect().parse::<Round>();
        let my_move = turn.pop().unwrap();
        let opponent_move = turn.pop().unwrap();
        total_score += match_score_part1(&opponent_move,&my_move);
        */
    }
    println!("Answer for Part One: {}",total_score);

    // Start of part two
    let mut total_score = 0;
    let list_matches = input.lines();
    for strategy in list_matches {
        let mut turn: String = strategy.split(' ').collect();
        let my_move = turn.pop().unwrap();
        let opponent_move = turn.pop().unwrap();
        total_score += match_score_part2(&opponent_move,&my_move);
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

