
// Rust

use std::fs;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::LinkedList;
use regex_lite::Regex;

pub fn boat_race(){
    // Linked list of array of bytes representing characters
    let mut list: LinkedList<crate::day6::race_utils::Race> = LinkedList::new();

    let file_path = Path::new("src/day6/data.txt");
    while let a = list.
    for iter in list {
        println!("{}", line);

        let race = Race {
            race_time: time,
            record_distance: button
        };
        list.push_back(
            race
        )
    }

    let mut iter = list.iter();
    let mut sum = 0;
    while let x = iter.next().unwrap() {
        let mut ways = 0;
        super::race_utils::ways_to_win(x, ways, 0);
        sum += ways;
    };

    println!("DAY6 sum: {} \n", sum);
}

// Loading file
fn read_File(file_path: &Path)-> LinkedList<LinkedList<String>> {
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
        println!("{}", line);
        list.push_back(tokenize(line, String::from(" "), true));
    }
    return list;
}

// Tokenize and cleaning spaces lines
fn tokenize(text: String, separator: String, cleaning_reg: Regex) -> LinkedList<String> {
    let mut list:LinkedList<String> = LinkedList::new();
    let mut splited = text.split(separator.as_str());
    for iter in splited {

        if iter == " " || iter == "\t" {
            continue;
        }

        if remove_duplicate {
            let cleaned = cleaning_reg.replace_all();
            list.push_back(iter.replace(" ","")); // Spaces cleaning - TODO: may panic 
        } else {
        list.push_back(iter.to_string());
        }
    }
    return list;
}