use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("../input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let processed_coordinates = convert_to_coordinates(ip.clone());
                let result = calculate_differences(processed_coordinates);
                println!("{:?}", result); 
            }
        }
    }
}

fn calculate_differences(coordinates: Vec<usize>) -> Vec<usize> {
    let mut processed_coordinates: Vec<usize> = Vec::new();
    for i in 1..coordinates.len() {
        let new_item = coordinates[i - 1] - coordinates[i];
        processed_coordinates.push(new_item);
    }
    
    if processed_coordinates.iter().all(|&x| x == 0) {
        return processed_coordinates;
    }
    else {
        return calculate_differences(processed_coordinates);
    }
}

fn convert_to_coordinates(line: String) -> Vec<usize> {
    let coordinates = line.split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
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