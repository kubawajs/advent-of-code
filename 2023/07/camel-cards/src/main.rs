use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Reverse;

fn main() {
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("../input.txt") {
        let card_list = vec![
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];
        let mut hands: Vec<(Vec<(&char, &i32)>, usize)> = Vec::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let hand = ip
                    .split(" ")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

                let hand_hash_map = into_character_map(&hand[0]);
                let mut cards_in_hand: Vec<_> = hand_hash_map.iter().collect();
                cards_in_hand.sort_unstable_by_key(|k| {
                    (
                        Reverse(k.1),
                        card_list
                            .iter()
                            .position(|&x| x == *k.0)
                            .unwrap(),
                    )
                });
                println!("{:?}", &cards_in_hand);

                let bid = hand[1].parse::<usize>().unwrap();
                let hand = (cards_in_hand, bid);
                // hands.push(hand.clone());
                println!("{:?}", hand.0);
            }
        }
    }
}

fn into_character_map(word: &str) -> HashMap<char, i32> {
    word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
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
