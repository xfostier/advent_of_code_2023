
// Rust

use std::fs;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::LinkedList;
use std::str::Bytes;
mod text_utils;


// ex: 1zefone5|zeekfour
fn digits_factory() {
    let mut iter = list.iter();
    let mut buffer_i = [u8];
    let mut buffer_j = [u8];
    let mut i = 0;
    let mut j = 0;
    while let x = iter.next() {
        text_utils::extract_number(x);
    };
}


fn read_file(file_path: Path) {
    // Linked list of array of bytes representing characters
    let mut list: LinkedList<&[u8]> = LinkedList::new();

    // Loading file
    let file_path = Path::new("src/day1/data.txt");
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        list.push_back(line.as_bytes());
    }
    return list;
}