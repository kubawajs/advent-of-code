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
        
        let mut currently_reading = "";
        for line in file_content.iter().skip(1) {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    continue;
                }
                else if ip.starts_with("seed-to-soil") {
                    currently_reading = "seed_to_soil_map";
                }
                else if ip.starts_with("soil-to-fertilizer") {
                    currently_reading = "soil_to_fertilizer_map";
                }
                else if ip.starts_with("fertilizer-to-water") {
                    currently_reading = "fertilizer_to_water_map";
                }
                else if ip.starts_with("water-to-light") {
                    currently_reading = "water_to_light_map";
                }
                else if ip.starts_with("light-to-temperature") {
                    currently_reading = "light_to_temperature_map";
                }
                else if ip.starts_with("temperature-to-humidity") {
                    currently_reading = "temperature_to_humidity_map";
                }
                else if ip.starts_with("humidity-to-location") {
                    currently_reading = "humidity_to_location_map";
                }
                else {
                    match currently_reading {
                        "seed_to_soil_map" => seed_to_soil_map.push(load_input(ip.clone())),
                        "soil_to_fertilizer_map" => soil_to_fertilizer_map.push(load_input(ip.clone())),
                        "fertilizer_to_water_map" => fertilizer_to_water_map.push(load_input(ip.clone())),
                        "water_to_light_map" => water_to_light_map.push(load_input(ip.clone())),
                        "light_to_temperature_map" => light_to_temperature_map.push(load_input(ip.clone())),
                        "temperature_to_humidity_map" => temperature_to_humidity_map.push(load_input(ip.clone())),
                        "humidity_to_location_map" => humidity_to_location_map.push(load_input(ip.clone())),
                        _ => continue,
                    }
                }
            }
        }

        let mut lowest_location: usize = usize::max_value();
        for seed in seeds {
            println!("Seed: {}", seed);

            // convert to soil
            let mut result = convert_by_map(seed, &seed_to_soil_map);
            println!("Soil: {}", seed);

            // convert to fertilizer
            result = convert_by_map(result, &soil_to_fertilizer_map);
            println!("Fertilizer: {}", result);

            // convert to water
            result = convert_by_map(result, &fertilizer_to_water_map);
            println!("Water: {}", result);

            // convert to light
            result = convert_by_map(result, &water_to_light_map);
            println!("Light: {}", result);

            // convert to temperature
            result = convert_by_map(result, &light_to_temperature_map);
            println!("Temperature: {}", result);

            // convert to humidity
            result = convert_by_map(result, &temperature_to_humidity_map);
            println!("Humidity: {}", result);

            // convert to location
            result = convert_by_map(result, &humidity_to_location_map);
            println!("Location: {}", result);

            if lowest_location > result {
                lowest_location = result;
            }
        }

        println!("Lowest location: {}", lowest_location);
    }
}

fn load_input(ip: String) -> Vec<usize> {
    let line = ip.split(" ").collect::<Vec<&str>>();
    return line.iter().map(|&x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
}

fn convert_by_map(value: usize, map: &Vec<Vec<usize>>) -> usize {
    for map_value in map {
        println!("Map value: {:?}", map_value);
        if (value >= map_value[1]) && (value < map_value[1] + map_value[2])
        {
            let diff = value - map_value[1];
            println!("Diff: {}", diff);
            return map_value[0] + diff;
        }
    }

    return value;
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