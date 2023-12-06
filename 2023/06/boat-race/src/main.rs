use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut times: Vec<usize> = Vec::new();
        let mut distances: Vec<usize> = Vec::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("Time") {
                    let times_line = ip.split(":").collect::<Vec<&str>>();
                    let times_str = times_line[1].split_whitespace().collect::<Vec<&str>>();
                    for time in times_str {
                        times.push(time.parse::<usize>().unwrap());
                    }
                }
                else if ip.starts_with("Distance") {
                    let distances_line = ip.split(":").collect::<Vec<&str>>();
                    let distances_str = distances_line[1].split_whitespace().collect::<Vec<&str>>();
                    for distance in distances_str {
                        distances.push(distance.parse::<usize>().unwrap());
                    }
                }
            }
        }

        let mut number_of_ways: Vec<usize> = Vec::new();
        for (index, time) in times.iter().enumerate() {
            let mut ways: usize = 0;
            let record_distance = distances[index];
            let mut charge_time = 0;
            
            while &charge_time < time {
                let run_time = time - charge_time;
                let current_distance = run_time * charge_time;
                if current_distance > record_distance {
                    ways += 1;
                }
                println!("Run time: {}, Charge time: {}, Distance: {}", run_time, charge_time, current_distance);
                charge_time += 1;
            }

            number_of_ways.push(ways);
        }

        let mut result: usize = 1;
        for way in number_of_ways {
            result *= way;
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
