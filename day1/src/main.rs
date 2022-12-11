use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Need file argument!");
        exit(1);
    }

    let elves = read_elves(&args[1]);
    if let Some(fattest) = elves.iter().max_by_key(|e| e.total()) {
        println!("Fattest Elf:  {} - {} Calories", fattest.id(), fattest.total());
    } else {
        println!("No elves found...");
    }
}

fn read_elves(file_name: &String) -> Vec<Elf> {
    let mut elves: Vec<Elf> = Vec::new();
    let mut current_elf = Elf::new(0);

    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            match line.unwrap().as_str() {
                "" => {
                    let new_id = current_elf.id() + 1;
                    elves.push(current_elf);
                    current_elf = Elf::new(new_id);
                }
                cals=> {
                    let calories: isize = cals.to_string().parse().unwrap();
                    current_elf.add_calories(calories)
                }
            }
        }
    }

    return elves;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Elf {
    id: isize,
    food: Vec<isize>
}

impl Elf {
    fn new(id: isize) -> Elf {
        Elf{id, food: Vec::new()}
    }

    fn id(&self) -> isize {
        self.id
    }

    fn add_calories(&mut self, calories: isize) {
        self.food.push(calories);
    }

    fn total(&self) -> isize {
        self.food.iter().sum()
    }
}