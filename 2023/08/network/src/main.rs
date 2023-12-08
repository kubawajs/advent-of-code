use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("../input.txt") {
        
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                
            }
        }

        
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
