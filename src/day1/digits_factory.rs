
// Rust

use std::fs;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::LinkedList;
use std::str::Bytes;
mod text_utils;

fn digits_factory() {

    // Linked list of array of bytes representing characters
    let mut list: LinkedList<&[u8]> = LinkedList::new();

    // Loading file
    let file_path = Path::new("day1/data.txt");
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        list.push_back(line.as_bytes());
    }

    let mut iter = list.iter();
    while let x = iter.next() {
        text_utils::extract_number(x);
    };
}