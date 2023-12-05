
// Rust

use std::fs;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::LinkedList;
mod text_utils;

fn main() -> std::io::Result<()> {

    let mut list: LinkedList<String> = LinkedList::new();
    let file_path = Path::new("data.txt");
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        list.push_back(line);
    }

    let mut iter = list.iter();
    while let x = iter.next() {
        validate(x)
    };

    Ok(())
}