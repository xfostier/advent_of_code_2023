
// Rust

pub mod text_utils {
    use std::{ops::Index, io::Bytes};

    pub fn extract_number(word: &[u8]) -> u8 {
        const MIN_SIZE:u8 = 3;
        const MAX_SIZE:u8 = 6;
        let mut i = 0 ;
        let mut j = word.len();
        let mut head:Box<[u8]>;
        let mut tail:Box<[u8]>;
        let mut checker:Box<[u8]>;

        while i < j {
            if validate_short(&word[i]) && !head.is_empty() {
                head = &[word[i]];
            } else {
                tail = [word[i]];
            };
        } ;
    }

    fn validate_short(value: &u8) -> bool {
        return *value > 050 && *value < 059;
    }

    fn validate_long(value: &[u8]) -> bool {
        const words: [&[u8]; 10] = [
            "zero".as_bytes(),
            "one".as_bytes(),
            "two".as_bytes(),
            "three".as_bytes(),
            "four".as_bytes(),
            "five".as_bytes(),
            "six".as_bytes(),
            "seven".as_bytes(),
            "height".as_bytes(),
            "nine".as_bytes()
        ];
        return words.contains(&value);
    }
}