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
                    },
                    None => {}
                }

                input.push(row.clone());
                println!("{:?}", row);
            }
        }

        println!("{:?}", start_directions);
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