use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let red_cubes_limit: i8 = 12;
    let green_cubes_limit: i8 = 13;
    let blue_cubes_limit: i8 = 14;

    let mut result: i32 = 0;

    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("../input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let game = ip.split(":").collect::<Vec<&str>>();
                let game_id = game[0].chars().collect::<Vec<char>>();
                let throws = game[1].split(";").collect::<Vec<&str>>();

                let mut valid_throw = true;
                for throw in throws {
                    let throw_without_spaces = throw.replace(" ", "");

                    for set in throw_without_spaces.split(",") {
                        if set.contains("red") {
                            let number_of_cubes = set.replace("red", "").parse::<i8>().unwrap();
                            valid_throw = number_of_cubes <= red_cubes_limit;
                            if !valid_throw {
                                break;
                            }
                        }
                        if set.contains("green") {
                            let number_of_cubes = set.replace("green", "").parse::<i8>().unwrap();
                            valid_throw = number_of_cubes <= green_cubes_limit;
                            if !valid_throw {
                                break;
                            }
                        }
                        if set.contains("blue") {
                            let number_of_cubes = set.replace("blue", "").parse::<i8>().unwrap();
                            valid_throw = number_of_cubes <= blue_cubes_limit;
                            if !valid_throw {
                                break;
                            }
                        }
                    }
                    if !valid_throw {
                        break;
                    }
                }

                if (valid_throw) {
                    let game_id_value: i32 = game_id
                        .iter()
                        .skip(5)
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();
                    println!("Game ID: {} \n", game_id_value);
                    result += game_id_value;
                }
            }
        }

        println!("Result: {}", result);
    }
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

