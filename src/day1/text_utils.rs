
// Rust

pub mod text_utils {

    use std::{ops::Index, io::Bytes};

    enum Matching {
        Full,
        Partial,
        None
    }

    pub fn extract_number(word: Vec<u8>) -> String {
        const MIN_SIZE:u8 = 3;
        const MAX_SIZE:u8 = 6;

        let mut head_buffer = Vec::new();
        let mut tail_buffer = Vec::new();
        let mut head = Vec::new();
        let mut tail = Vec::new();
        let mut i = 0;
        let mut j = word.len();

        while i < (word.len()/2 + usize::from(MAX_SIZE)) {

            head_buffer.append(word[i]);

            // simple head
            if validate_short(&word[i]) && head.is_empty() {
                if head.is_empty() {
                    head = &[word[j]];
                } else{
                    tail = &[word[j]];
                };
            };

            // simple tail
            if validate_short(&word[j]) {
                if tail.is_empty() {
                    tail = &[word[j]];
                } else{
                    head = &[word[j]];
                };
            };

            if (!head.is_empty() && !tail.is_empty()) {
                return [head, tail].concat()
            }
        }
    }

    fn validate_short(value: &u8) -> Matching {
        if *value > 050 && *value < 059 {
            return Matching.Full;
        }
    }

    fn validate(value: [u8]) -> Matching {

        if value.len() == 1 {

        }

        let three: [[u8]; 10] = [
            
            "one".as_bytes(),
            "two".as_bytes(),
            "three".as_bytes(),
            
            
            "six".as_bytes(),
            "seven".as_bytes(),
            "height".as_bytes(),
            
        ];
        let four: [[u8]; 4] = [
            "zero".as_bytes(),
            "four".as_bytes(),
            "five".as_bytes(),
            "nine".as_bytes()
        ]

        const five:

        const six:

        
        return words.contains(&value);
    }
}
