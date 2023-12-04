use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result: i32 = 0;

    if let Ok(lines) = read_lines("../input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut current_card_result: i32 = 0;
            if let Ok(ip) = line {
                let card = ip.split(":").collect::<Vec<&str>>();
                let numbers_set = card[1].split("|").collect::<Vec<&str>>();
                let winning_numbers = numbers_set[0].split_whitespace().collect::<Vec<&str>>();
                let player_numbers = numbers_set[1].split_whitespace().collect::<Vec<&str>>();

                let common_numbers = winning_numbers
                    .iter()
                    .filter(|&n| player_numbers.contains(n))
                    .collect::<Vec<&&str>>();

                for (index, _number) in common_numbers.iter().enumerate() {
                    if index == 0 {
                        current_card_result = 1;
                    }
                    else {
                        current_card_result *= 2;
                    }
                }

                println!("Card result: {}", current_card_result);
                result += current_card_result;
            }
        }
    }

    println!("Result: {}", result);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
