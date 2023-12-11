use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("../input.txt") {
        let mut input = Vec::new();
        let mut start_directions: (i32, i32) = (0, 0);

        // Consumes the iterator, returns an (Optional) String
        for (index, line) in lines.enumerate() {
            if let Ok(ip) = line {
                let row = ip.chars().collect::<Vec<char>>();
                match ip.find('S') {
                    Some(s) => {
                        start_directions = (index as i32, s as i32);
                    }
                    None => {}
                }

                println!("{:?}", row);
                input.push(row.clone());
            }
        }

        println!("Start directions: {:?}", start_directions);
        
        let mut current_position = start_directions;
        let mut current_path = vec![current_position];
        let current_item = input[current_position.0 as usize][current_position.1 as usize];
        let possible_moves = find_possible_moves(current_position, current_item);
        let mut valid_moves = check_indexes(current_position, possible_moves.clone(), input.len() as i32, input[0].len() as i32);

        println!("Possible moves in 1st move: {:?}", valid_moves);

        let mut directions: Vec<(i32, i32)> = Vec::new();
        let directions = calculate_directions(valid_moves, current_position, input.clone(), start_directions, start_directions);

        println!("Directions: {:?}", directions);
    }
}

fn calculate_directions(valid_moves: Vec<(i32, i32)>, current_position: (i32, i32), input: Vec<Vec<char>>, start_directions: (i32, i32), previous_step: (i32, i32)) -> Vec<(i32, i32)> {
    let mut directions: Vec<(i32, i32)> = Vec::new();
    for move_ in valid_moves {
        if move_ == start_directions {
            println!("Found finish: {:?}", start_directions);
            return directions;
        }
        let item = input[move_.0 as usize][move_.1 as usize];
        let mut new_moves = find_possible_moves(current_position, item);
        new_moves = check_indexes(previous_step, new_moves, input.len() as i32, input[0].len() as i32);      
        if new_moves.len() > 0 {
            println!("Current position: {:?}", current_position);
            println!("Chosen move: {:?}", move_);
            println!("Possible moves in next move: {:?}", new_moves);
            directions.push(move_);
            calculate_directions(new_moves, move_, input.clone(), start_directions, current_position);
            break;
        }
    }

    return directions;
}

fn find_possible_moves(current_position: (i32, i32), current_item: char) -> Vec<(i32, i32)> {
    let mut valid_moves = Vec::new();
    match current_item {
        '.' => {
        }
        'S' => {
            valid_moves.push((current_position.0 - 1, current_position.1));
            valid_moves.push((current_position.0 + 1, current_position.1));
            valid_moves.push((current_position.0, current_position.1 - 1));
            valid_moves.push((current_position.0, current_position.1 + 1));
        }
        '|' => {
            valid_moves.push((current_position.0 - 1, current_position.1));
            valid_moves.push((current_position.0 + 1, current_position.1));
        }
        '-' => {
            valid_moves.push((current_position.0, current_position.1 - 1));
            valid_moves.push((current_position.0, current_position.1 + 1));
        }
        'L' => {
            valid_moves.push((current_position.0 - 1, current_position.1));
            valid_moves.push((current_position.0, current_position.1 + 1));
        }
        'J' => {
            valid_moves.push((current_position.0 - 1, current_position.1));
            valid_moves.push((current_position.0, current_position.1 - 1));
        }
        '7' => {
            valid_moves.push((current_position.0 + 1, current_position.1));
            valid_moves.push((current_position.0, current_position.1 - 1));
        }
        'F' => {
            valid_moves.push((current_position.0 + 1, current_position.1));
            valid_moves.push((current_position.0, current_position.1 + 1));
        }
        _ => {}
    }

    return valid_moves;
}

fn check_indexes(previous_step: (i32, i32), indexes_to_check: Vec<(i32, i32)>, row_len: i32, col_len: i32) -> Vec<(i32, i32)> {
    return indexes_to_check.iter().filter(|&x| x.0 >= 0 && x.0 < row_len && x.1 >= 0 && x.1 < col_len && !(x.0 == previous_step.0 && x.1 == previous_step.1)).map(|&x| x).collect::<Vec<(i32, i32)>>();
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
