use wordsearch::wordsearch;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let w = wordsearch::WordSearch::new(&[
        "ability",
        "able",
        "about",
        "above",
        "accept",
        "according",
        "account",
        "across",
        "act",
        "action",
        "activity",
        "actually",
        "add",
        "address",
        "administration",
        "admit",
        "adult",
        "affect",
        "after",
        "again",
        "against",
    ], false, 100);
    println!("{}", w.unwrap());
}
