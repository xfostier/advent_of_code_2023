
// Rust

use std::fs;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::LinkedList;
mod race_utils;

pub fn boat_race(){
    // Linked list of array of bytes representing characters
    let mut list: LinkedList<Race> = LinkedList::new();

    // Loading file
    const file_path = Path::new("data.txt");
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let splited = line.split(' ');
        for s in splited {
            list.push_back(
                Race {
                    race_time: s[1].replace(" ",""), // Spaces cleaning
                    record_distance: s[2].replace(" ","") // Spaces cleaning
                }
            )
        }
    }

    let mut iter = list.iter();
    let mut sum = 0;
    while let x = iter.next(){
        let mut ways = 0;
        race_utils::ways_to_win(iter, &ways, 0);
        sum += ways;
    };
}