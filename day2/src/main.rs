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

    fn winning_move(self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock ,
        }
    }

    fn losing_move(self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper ,
        }
    }

    fn drawing_move(self) -> Self {
        self
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

// Section below is commented out because it was moved to main()
// It was not possible to use this trait while solving both parts 1 and 2 in a single file
/* 
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
 */

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

    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Win =>  theirs.winning_move(),
            Outcome::Draw => theirs.drawing_move(),
            Outcome::Loss => theirs.losing_move(),
        }

    }
}

impl TryFrom<char> for Outcome {
    type Error = &'static str;

    fn try_from(c:char) -> Result<Self,Self::Error> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("Invalid outcome")
        }
    }
}

fn main() {
    println!("Advent of Code 2022 - Day 2");

    let input = open_input_file();

    let mut total_score = 0;
    let list_matches = input.lines();

    for round_str in list_matches {
        // Parsing string into a Round struct
        let mut chars = round_str.chars();
        let (Some(theirs), Some(' '), Some(ours),None) = 
        (chars.next(),chars.next(),chars.next(),chars.next()) else {
            panic!("Round does not follow format <theirs>SP<ours>EOF");
        }; 
        // Round is destructured above here in main instead of trait FromStr
        // This is done so that both parts of the puzzle can be solved in a single file
        // 

        let round = Round {
            theirs: theirs.try_into().unwrap(),
            ours: ours.try_into().unwrap(),
        };
        let round_score = round.our_score();
        total_score += round_score;
    }
    println!("Answer for Part One: {}",total_score);

    // Start of part two
    let mut total_score = 0;
    let list_matches = input.lines();
    for round_str in list_matches {
        let mut chars = round_str.chars();
        let (Some(theirs), Some(' '), Some(outcome),None) = 
        (chars.next(),chars.next(),chars.next(),chars.next()) else {
            panic!("Round does not follow format <theirs>SP<outcome>EOF");
        };

        let theirs = Move::try_from(theirs).unwrap();
        let outcome = Outcome::try_from(outcome).unwrap();
        let ours = outcome.matching_move(theirs);
        let round = Round {
            theirs: theirs,
            ours: ours,
        };
        let round_score = round.our_score();
        total_score += round_score;
    }

    println!("Answer for Part Two: {}",total_score);
}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}

