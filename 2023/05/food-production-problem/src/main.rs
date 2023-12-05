use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file = File::open("../input.txt").unwrap();
    let mut reader = io::BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let mut seeds: Vec<usize> = read_seeds(&line);
    println!("Seeds: {:?}", seeds);
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