
// Rust

use std::fs;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::LinkedList;
use crate::day6::race_utils::Race;

pub fn boat_race(){
    // Linked list of array of bytes representing characters
    let mut list: LinkedList<crate::day6::race_utils::Race> = LinkedList::new();

    // Loading file
    let file_path = Path::new("data.txt");
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!("Failure when reading line from buffer: {:?}", error),
        };
        let splited = line.split(' ');
        let a = splited.nth(1).unwrap().replace(" ","").parse::<u8>().unwrap();
        let b = splited.nth(0).unwrap().replace(" ","").parse::<u8>().unwrap();
        let race = Race {
            race_time: a, // Spaces cleaning
            record_distance: b, // Spaces cleaning
        };
        list.push_back(
            race
        )
    };

    let mut iter = list.iter();
    let mut sum = 0;
    while let x = iter.next(){
        let mut ways = 0;
        super::race_utils::ways_to_win(x.unwrap().to_owned(), &ways, 0);
        sum += ways;
    };

    println!("DAY6 sum: {} \n", sum);
}