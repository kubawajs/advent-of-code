use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("../input.txt") {
        // Consumes the iterator, returns an (Optional) String\
        let mut added_values: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                let processed_coordinates = convert_to_coordinates(ip.clone());
                let result: Vec<Vec<i32>> = Vec::new();
                let process = calculate_differences(processed_coordinates, result);                
                let mut last_value: i32 = 0;
                
                for i in 1..process.len() {
                    last_value = last_value + process[i].last().unwrap();
                }

                added_values.push(last_value);
            }
        }

        let final_result: i32 = added_values.iter().sum();
        println!("Final result: {}", final_result);
    }
}

fn calculate_differences(coordinates: Vec<i32>, mut result: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    result.push(coordinates.clone());
    let mut processed_coordinates: Vec<i32> = Vec::new();
    for i in 1..coordinates.len() {
        let new_item = coordinates[i] - coordinates[i - 1];
        processed_coordinates.push(new_item);
    }
    
    //println!("{:?}", processed_coordinates);
    if processed_coordinates.iter().all(|&x| x == 0) {
        result.push(processed_coordinates.clone());
        result.reverse();
        return result;
    }
    else {
        return calculate_differences(processed_coordinates, result);
    }
}

fn convert_to_coordinates(line: String) -> Vec<i32> {
    let coordinates = line.split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    return coordinates;
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