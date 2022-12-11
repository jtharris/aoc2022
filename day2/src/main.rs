use std::fs::File;
use std::{env, io};
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Need file argument!");
        exit(1);
    }
    let score = score_file(&args[1]);

    println!("Total score:  {}", score);
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn score_file(file_name: &String) -> isize {
    match read_lines(file_name) {
        Ok(lines) => {
            lines.map(|l| score_line(l.unwrap().as_str())).sum()
        }
        Err(e) => {
            println!("Error reading file:  {}", e);
            0
        }
    }
}

fn score_line(line: &str) -> isize {
    if line.len() != 3 {
        panic!("Expecting line of length 3:  {}", line);
    }

    let chars: Vec<char> = line.chars().collect();


    // Phase 1 logic
//    let opponent = Shape::parse(chars[0]);
//    let player = Shape::parse(chars[2]);

//    let outcome = Outcome::vs(&opponent, &player);
//    let score = outcome.score() + player.score();

    // Phase 2 logic
    let opponent = Shape::parse(chars[0]);
    let outcome = Outcome::parse(chars[2]);
    let player = outcome.find_match_shape(opponent);

    let score = outcome.score() + player.score();

    // debugging
    println!("{}:  {}", line, score);

    score
}

enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn parse(input: char) -> Shape {
        match input {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            unrecognized => panic!("Unknown Shape character:  {}", unrecognized)
        }
    }

    fn score(&self) -> isize {
       match self {
           Shape::Rock => 1,
           Shape::Paper => 2,
           Shape::Scissors => 3
       }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Rock => write!(f, "Rock"),
            Shape::Paper => write!(f, "Paper"),
            Shape::Scissors => write!(f, "Scissors")
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw
}

impl Outcome {
    fn vs(opponent: &Shape, player: &Shape) -> Outcome {
       match (opponent, player) {
           (Shape::Rock, Shape::Paper) => Outcome::Win,
           (Shape::Rock, Shape::Scissors) => Outcome::Lose,
           (Shape::Paper, Shape::Rock) => Outcome::Lose,
           (Shape::Paper, Shape::Scissors) => Outcome::Win,
           (Shape::Scissors, Shape::Rock) => Outcome::Win,
           (Shape::Scissors, Shape::Paper) => Outcome::Lose,
           _ => Outcome::Draw
       }
    }

    fn parse(input: char) -> Outcome {
        match input {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            unrecognized => panic!("Unknown Outcome character:  {}", unrecognized)
        }
    }

    fn find_match_shape(&self, opponent: Shape) -> Shape {
        match (self, opponent) {
            (Outcome::Win, Shape::Paper) => Shape::Scissors,
            (Outcome::Win, Shape::Rock) => Shape::Paper,
            (Outcome::Win, Shape::Scissors) => Shape::Rock,
            (Outcome::Lose, Shape::Paper) => Shape::Rock,
            (Outcome::Lose, Shape::Rock) => Shape::Scissors,
            (Outcome::Lose, Shape::Scissors) => Shape::Paper,
            (Outcome::Draw, other) => other
        }
    }

    fn score(&self) -> isize {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6
        }
    }
}