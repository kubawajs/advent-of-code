use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut seed_to_soil_map: Vec<Vec<usize>> = Vec::new();
        let mut soil_to_fertilizer_map: Vec<Vec<usize>> = Vec::new();
        let mut fertilizer_to_water_map: Vec<Vec<usize>> = Vec::new();
        let mut water_to_light_map: Vec<Vec<usize>> = Vec::new();
        let mut light_to_temperature_map: Vec<Vec<usize>> = Vec::new();
        let mut temperature_to_humidity_map: Vec<Vec<usize>> = Vec::new();
        let mut humidity_to_location_map: Vec<Vec<usize>> = Vec::new();

        let file_content = lines.collect::<Vec<Result<String, io::Error>>>();
        let seeds = read_seeds(file_content[0].as_ref().unwrap());
        
        let mut current_index: usize = 1;
        while true {
            let current_line = file_content[current_index].as_ref().unwrap();
            current_index += 1;
            if current_line.is_empty() == true {
                break;
            }
            
            if current_line.contains(":") {
                continue;
            }

            let map = current_line.split_whitespace().collect::<Vec<&usize>>();
            seed_to_soil_map.push(map);
        }

        println!("Seed to soil map: {:?}", seed_to_soil_map);
        println!("Seeds: {:?}", seeds);
    }
}

fn read_seeds(ip: &str) -> Vec<usize> {
    let seeds_line = ip.split(":").collect::<Vec<&str>>();
    let seeds_str = seeds_line[1].trim();
    let mut seeds: Vec<usize> = Vec::new();

    for seed in seeds_str.split(" ") {
        seeds.push(seed.parse::<usize>().unwrap());
    }

    return seeds;
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