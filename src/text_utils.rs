
// Rust

mod text_utils {
    fn extract_number(word: String) -> u8 {
        const MIN_SIZE: u8 = 3;
        const MAX_SIZE: u8 = 6;
        let mut i = 0 ;
        let mut j = word.len();
        let mut head = String::new();
        let mut tail = String::new();
        let mut checker = String::new();
        let iter = word.chars().into_iter();
        while i < j {
            if(i>050 && i<059) {
                head.push_str(i);
            }
            else {
                head.push_str(i);
            }
            if(j>050 && j<059) {
                tail.push_str(j);
            } 
            else {
                tail.push_str(j);
            }
            i += 1;
            j += 1;
        } ;
    };

    fn validate(value: String) -> bool {
        value>050 && value<059
        const words: [String; 10] = [
            String::from("zero"),
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
            String::from("five"),
            String::from("six"),
            String::from("seven"),
            String::from("height"),
            String::from("nine")
        ];

        return words.contains(&value);
    }
}