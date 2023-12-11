use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("../input.txt") {
        let mut galaxy_map = Vec::new();
        let mut columns_with_galaxies: Vec<i32> = Vec::new();
        
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let row = ip.chars().collect::<Vec<char>>();
                if ip.chars().all(|x| x == '.') {
                    // duplicate the row if no galaxies are present
                    galaxy_map.push(row.clone());
                }
                else {
                    for (i, c) in row.iter().enumerate() {
                        if *c == '#' && columns_with_galaxies.contains(&(i as i32)) == false {
                            columns_with_galaxies.push(i as i32);
                        }
                    }
                }

                galaxy_map.push(row.clone());
            }
        }

        columns_with_galaxies.sort();
        let mut adjusted_galaxy_map = adjust_galaxy_map(galaxy_map.clone(), &columns_with_galaxies);

        let mut galaxies_coords = get_galaxy_coords(adjusted_galaxy_map.clone());

        println!("{:?}", galaxies_coords);

        let mut iterator = 0;
        let mut overall_distance = 0;

        loop {
            for i in iterator..galaxies_coords.len() - 1 {
                let distance = calculate_distance(galaxies_coords[iterator], galaxies_coords[i + 1]);
                overall_distance += distance;
                println!("Distance {:?}-{:?}: {}", galaxies_coords[iterator], galaxies_coords[i + 1], distance);
            }

            if iterator == galaxies_coords.len() - 1 {
                break;
            }
            iterator += 1;
        }

        println!("Overall distance: {}", overall_distance);
    }
}

fn calculate_distance (galaxy1: (i32, i32), galaxy2: (i32, i32)) -> i32 {
    let distance = (galaxy1.0 - galaxy2.0).abs() + (galaxy1.1 - galaxy2.1).abs();
    distance
}

fn get_galaxy_coords (galaxy_map: Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut galaxies_coords = Vec::new();
    for (index, row) in galaxy_map.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies_coords.push((index as i32, i as i32));
            }   
        }
    }
    galaxies_coords
}

fn adjust_galaxy_map (galaxy_map: Vec<Vec<char>>, columns_with_galaxies: &Vec<i32>) -> Vec<Vec<char>> {
    let mut adjusted_galaxy_map: Vec<Vec<char>> = Vec::new();
    for old_row in galaxy_map {
        let mut new_row: Vec<char> = Vec::new();
        for (index, cell) in old_row.iter().enumerate() {
            new_row.push(cell.clone());

            // Duplicate the column if no galaxies are present
            if columns_with_galaxies.contains(&(index as i32)) == false {
                new_row.push(cell.clone());
            }
        }

        adjusted_galaxy_map.push(new_row);
    }
    adjusted_galaxy_map  
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
