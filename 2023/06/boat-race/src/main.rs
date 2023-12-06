use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut times: Vec<usize> = Vec::new();
        let mut times_concat: usize = 0;
        let mut distances: Vec<usize> = Vec::new();
        let mut distances_concat: usize = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("Time") {
                    times = load_input(ip.clone());
                    times_concat = load_input_concat(ip.clone());
                }
                else if ip.starts_with("Distance") {
                    distances = load_input(ip.clone());
                    distances_concat = load_input_concat(ip.clone());
                }
            }
        }

        let number_of_ways = calculate_race_ways(times, distances);
        let result = calculate_result(number_of_ways);
        println!("Result part 1: {}", result);

        let number_of_ways_concat = calculate_way(times_concat, distances_concat);
        println!("Result part 2: {}", number_of_ways_concat);
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

fn load_input(ip: String) -> Vec<usize> {
    let mut values: Vec<usize> = Vec::new();
    let line = ip.split(":").collect::<Vec<&str>>();
    let line_str = line[1].split_whitespace().collect::<Vec<&str>>();
    for line in line_str {
        values.push(line.parse::<usize>().unwrap());
    }

    return values;
}

fn calculate_result(number_of_ways: Vec<usize>) -> usize {
    let mut result: usize = 1;
    for way in number_of_ways {
        result *= way;
    }

    return result;
}

fn calculate_race_ways(times: Vec<usize>, distances: Vec<usize>) -> Vec<usize> {
    let mut number_of_ways: Vec<usize> = Vec::new();
    for (index, time) in times.iter().enumerate() {
        let ways = calculate_way(*time, distances[index]);
        number_of_ways.push(ways);
    }
    
    return number_of_ways;
}

fn calculate_way(time: usize, distance: usize) -> usize {
    let mut ways: usize = 0;
    let record_distance = distance;
    let mut charge_time = 0;
        
    while charge_time < time {
        let run_time = time - charge_time;
        let current_distance = run_time * charge_time;
        if current_distance > record_distance {
            ways += 1;
        }
        // println!("Run time: {}, Charge time: {}, Distance: {}", run_time, charge_time, current_distance);
        charge_time += 1;
    }

    return ways;
}

fn load_input_concat(ip: String) -> usize {
    let line = ip.split(":").collect::<Vec<&str>>();
    let line_str = line[1].replace(" ", "").parse::<usize>().unwrap();

    return line_str;
}