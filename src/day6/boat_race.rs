
// Rust

use std::fs;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::LinkedList;
// https://docs.rs/regex-lite/latest/regex_lite/
use regex_lite::Regex;

use crate::day6::race_utils;

use super::utils::Contatenable;

// PART 1
pub fn boat_race_part1(file_path: &Path) -> u64 {
    // Linked list of Races
    let mut list: LinkedList<crate::day6::race_utils::Race> = LinkedList::new();
    let dual_list = read_file(file_path);

    // https://stackoverflow.com/questions/38826633/how-to-skip-the-first-n-items-of-an-iterator-in-rust
    let mut times_iter = dual_list.iter().nth(0).unwrap().iter().skip(1); // dropping title
    let mut records_iter = dual_list.iter().nth(1).unwrap().iter().skip(1); // dropping title

    while let Some(time) = times_iter.next() {
        let record = records_iter.next().unwrap(); // TODO may panic
        let race = race_utils::Race {
            race_time: time.parse::<u64>().unwrap(),
            record_distance: record.parse::<u64>().unwrap()
        };
        
        list.push_back(
            race
        )
    }

    let mut iter = list.iter();
    let mut product = 1;
    while let Some(x) = iter.next() {
        let ways = 0;
        let ways = super::race_utils::ways_to_win_simple_recursive(x, ways, 0);
        product *= ways;
    };

    return product;
}

// PART 2
pub fn boat_race_part2(file_path: &Path) -> u64 {
    let mut dual_list = read_file(file_path);

    // https://stackoverflow.com/questions/38826633/how-to-skip-the-first-n-items-of-an-iterator-in-rust
    let time = dual_list.iter_mut().nth(0).unwrap().split_off(1).concatenated(); // dropping title
    let record = dual_list.iter_mut().nth(1).unwrap().split_off(1).concatenated(); // dropping title
    let race = race_utils::Race {
            race_time: time.parse::<u64>().unwrap(),
            record_distance: record.parse::<u64>().unwrap()
        };
    let ways = super::race_utils::ways_to_win_simple_recursive(&race, 0, 0);
    return ways;
}

// Loading file
fn read_file(file_path: &Path)-> LinkedList<LinkedList<String>> {
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let reader = BufReader::new(file);

    let mut list:LinkedList<LinkedList<String>> = LinkedList::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!("Failure when reading line from buffer: {:?}", error),
        };
        list.push_back(tokenize(line.as_str(), " ", Regex::new("[^\\s]+").unwrap()));
    }
    return list;
}

// Tokenize and cleaning spaces lines
fn tokenize<'a>(text: & 'a str, separator: &str, cleaning_reg: Regex) -> LinkedList<String> {
    let mut list:LinkedList<String> = LinkedList::new();
    let splited = text.split(separator);
    for iter in splited {

        if iter == " " || iter == "\t" || iter == "" {
            continue;
        }

        let match_range = cleaning_reg.find(iter).unwrap(); // Spaces cleaning - TODO: may panic 
        list.push_back(iter[match_range.start()..match_range.end()].to_string());
    }
    return list;
}