use wordsearch::wordsearch;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let w = wordsearch::WordSearch::new(&["Hello", "World"], false, 5);
    println!("{}", w.unwrap());
}
