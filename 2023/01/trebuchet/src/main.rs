use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result: i32 = 0;

    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("../input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut calc_value: [char; 2] = ['0', '0'];
                for item in ip.chars() {
                    if item.is_numeric() {
                        calc_value[0] = item;
                        break;
                    }
                }
                for item in ip.chars().rev() {
                    if item.is_numeric() {
                        calc_value[1] = item;
                        let value: i32 = calc_value
                            .iter()
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap();
                        result += value;
                        break;
                    }
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
