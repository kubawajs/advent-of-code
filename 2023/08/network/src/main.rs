use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("../input.txt") {
        let mut directives = Vec::new();
        let mut network = HashMap::new();

        // Consumes the iterator, returns an (Optional) String
        for (index, line) in lines.enumerate() {
            if let Ok(ip) = line {
                if index == 0 {
                    directives = convert_to_directives(ip.clone());
                    continue;
                }
                if index == 1 {
                    continue;
                }

                let (key, translation) = convert_to_network_item(ip.clone());
                network.insert(key, translation);
            }
        }

        let mut steps = 0;
        let mut directive_number = 0;
        let mut current_step = "AAA".to_string();
        loop {
            let dir_move = directives[directive_number];
            let next_step = network.get(&current_step).unwrap();
            current_step = next_step.iter().nth(dir_move as usize).unwrap().clone();

            steps += 1;
            if current_step == "ZZZ" {
                break;
            }
            if directive_number < directives.len() - 1 {
                directive_number += 1;
            }
            else {
                directive_number = 0;
            }
        }

        println!("Steps: {}", steps);
    }
}

fn convert_to_directives(line: String) -> Vec<u32> {
    line.replace("R", "1")
        .replace("L", "0")
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()
}

fn convert_to_network_item(line: String) -> (String, Vec<String>) {
    let binding_directive = line.replace(" ", "");
    let directive = binding_directive.split("=").collect::<Vec<&str>>();
    let key = directive.first().unwrap();
    let parsed_translation = directive.last().unwrap().replace("(", "").replace(")", "");
    let translation = parsed_translation.split(",").collect::<Vec<&str>>();
    (
        key.to_string(),
        translation.iter().map(|x| x.to_string()).collect(),
    )
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
