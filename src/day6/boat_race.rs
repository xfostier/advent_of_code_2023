
// Rust

use std::fs;
use std::io::{BufReader, BufRead};
use std::ops::Deref;
use std::path::Path;
use std::str;
use std::collections::LinkedList;
use crate::day6::race_utils::Race;

pub fn boat_race(){
    // Linked list of array of bytes representing characters
    let mut list: LinkedList<crate::day6::race_utils::Race> = LinkedList::new();

    // Loading file
    let file_path = Path::new("src/day6/data.txt");
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
        println!("{}", line);
        let mut splited = line.split(" ");
        for iter in splited.clone() {
            println!("HELLO {}", iter);
        }
        let time:u32 = splited.nth(1).unwrap().replace(" ","").parse::<u32>().unwrap(); // Spaces cleaning - TODO: may panic 
        let button:u32 = splited.nth(0).unwrap().replace(" ","").parse::<u32>().unwrap(); // Spaces cleaning - TODO: may panic 
        let race = Race {
            race_time: time,
            record_distance: button
        };
        list.push_back(
            race
        )
    };

    let mut iter = list.iter();
    let mut sum = 0;
    while let x = iter.next().unwrap() {
        let mut ways = 0;
        super::race_utils::ways_to_win(x, ways, 0);
        sum += ways;
    };

    println!("DAY6 sum: {} \n", sum);
}

fn readFile(file_path: &Path)-> LinkedList<T> {

}

fn sanitize(){

}