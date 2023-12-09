use std::collections::LinkedList;
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
pub trait Contatenable {
    type T;
    fn concatenated(&self) -> Self::T;
}

impl Contatenable for LinkedList<String> {
    type T = String;
    fn concatenated(&self) -> String {
        let mut result = String::new();
        let mut iter = self.iter();
        while let Some(x) = iter.next() {
            result.push_str(x);
        }
        return result;
    }
}