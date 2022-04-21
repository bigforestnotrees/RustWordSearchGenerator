use std::collections::{HashMap, HashSet};
use std::ops::Index;
use rand::rngs::SmallRng;
use rand::{SeedableRng, Rng};
use regex::Regex;

type X = i64;
type Y = i64;
type Point = (X, Y);
type Direction = (X, Y);
struct Word { 
    position:Point,
    direction: Direction,
    word: String,
}


const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIRSET: &[(X, Y);8] = &[(1,0),(1,-1),(0,-1),(-1,-1),(-1,0),(-1,1),(0,1), (1,1)];

#[derive(Debug, Clone)]
pub struct WordSearch {
    board: HashMap<(X, Y), char>,
}

impl WordSearch {
    pub fn new(words: &[&str], seeded: bool, size: i64) -> Option<Self> {
        if size < 5 || words.len() > (size as usize).pow(2) {
            return None;
        }

        if let Some(value) = validation(words, size) {
            return value;
        }

        let mut rng: SmallRng;
        if seeded {
            rng = SmallRng::from_entropy()
        } else {
            rng = SmallRng::from_seed([0; 32])
        }

        let word_search = WordSearch {
            board: (
                (0..size).map(|a| {
                    (0..size).map(|b| {
                        let idx = rng.gen_range(0..CHARSET.len());
                        ((b as X, a as Y), CHARSET[idx] as char)
                    }).collect::<Vec<((i64,i64), char)>>()
                }).flatten().collect()
            ),
        };

        // #FORESTBASED
        for y in 0..size {
            for x in 0..size {
                print!("{} ", word_search.board.get(&(x, y)).unwrap());
            }
            println!();
        }
        
        let mut words_by_len: Vec<String> = Vec::new();
        for s in words {
            words_by_len.push(s.to_uppercase());
        }

        words_by_len.sort();
        // Force it to be uppercase
        words_by_len = words_by_len.into_iter().rev().collect();

        let mut board = WordSearch { 
            board: HashMap::<(X,Y),char>::new()
        };

        let _ = (0..size).map(|x|{
            let _ = (0..size).map(|y| {
                board.board.insert((x, y), ' ')
            });
            ()
        });

        let mut added_words = Vec::<Word>::new();

        while !words_by_len.is_empty() {
            let current_word = words_by_len.get(0)?.clone();

            if current_word.is_empty() {
                break;
            }

            let mut possible_intersected_words = Vec::<Word>::new();

            for added_word in &added_words {
                let mut collision_positions = Vec::<Point>::new();

                let pos = added_word.position;
                let dir = added_word.direction;

                let mut collision_chars = HashSet::<char>::new();

                for c in current_word.chars().collect::<HashSet<char>>() {
                    for (i, c2) in added_word.word.chars().enumerate() {
                        if c == c2 {
                            let new_point = (pos.0 + dir.0 * i as i64, pos.1 + dir.1 * i as i64);
                            collision_positions.push(new_point);
                            collision_chars.insert(c);
                        }
                    }
                }

                let mut possible_dirs = Vec::<Direction>::new();
                let mut possible_pos = Vec::<Point>::new();

                for point in collision_positions {
                    for (i, c) in current_word.chars().enumerate() {
                        if collision_chars.contains(&c) && (&board).board.contains_key(&point) {
                            let before = i as i64;
                            let after = (current_word.len() - (i + 1)) as i64;

                            let opposite = (-added_word.direction.0, -added_word.direction.1);
                            let dir_set = DIRSET.into_iter().filter(|a| {
                                a.0 != added_word.direction.0 && a.0 != opposite.0 && a.1 != added_word.direction.1 && a.1 != opposite.1
                            });
                            for dir in dir_set {
                                let beginning_x = point.0 + before * -dir.0;
                                let beginning_y = point.1 + before * -dir.1;
                                let ending_x = point.0 + after * dir.0;
                                let ending_y = point.1 + after * dir.1;

                                if beginning_x >= 0 && beginning_x < size && 
                                beginning_y >= 0 && beginning_y < size &&
                                ending_x >= 0 && ending_x < size &&
                                ending_y >= 0 && ending_y < size {
                                    possible_pos.push((beginning_x, beginning_y));
                                    possible_dirs.push(*dir);
                                }
                            }
                        }
                    }
                }

                let mut to_remove = Vec::<usize>::new();

                for (i, pos) in (&possible_pos).into_iter().enumerate() {
                    for j in 0..current_word.len() {
                        let x = pos.0 + j as i64 * possible_dirs.get(i)?.0;
                        let y = pos.1 + j as i64 * possible_dirs.get(i)?.1;

                        if (&board).board.contains_key(&(x,y))
                            && (&board).board.get(&(x,y))? != &' ' 
                            && (&board).board.get(&(x,y))? != &current_word.chars().nth(j).unwrap()
                            && !to_remove.contains(&i) {
                                to_remove.push(i);
                        }
                    }
                }

                for i in to_remove.into_iter().rev() {
                    possible_dirs.remove(i);
                    possible_pos.remove(i);
                }

                if possible_dirs.len() != possible_pos.len() {
                    return None;
                }

                if !possible_pos.is_empty() {
                    for (i, pos) in (&possible_pos).into_iter().enumerate() {
                        possible_intersected_words.push(Word { 
                            word: current_word.clone(),
                            position: *pos,
                            direction: *possible_dirs.get(i)?,
                        });
                    }
                }
            }


            let end: i64= current_word.len() as i64 - 1;
            let mut possible_single_words = Vec::<Word>::new();

            for dir in DIRSET {
                for ((x,y), _) in &board.board {
                    if x + end * dir.0 > size - 1 || y + end * dir.1 > size -1
                        || x + end * dir.0 < 0 || y + end * dir.1 < 0 {
                            continue;
                    }
                    else {
                        let mut broke = false;
                        for i in 0..current_word.len() {
                            if board.board.contains_key(&(x + i as i64 * dir.0, y + i as i64 * dir.1)) {
                                broke = true;
                                break;
                            }
                        }
                        if !broke {
                            possible_single_words.push(Word { position: (*x,*y), direction: *dir, word: current_word.clone() })
                        }
                    }
                }
            }

            let choice: Word;

            if !possible_intersected_words.is_empty() && rng.gen_bool(0.5) {
                let temp = possible_intersected_words.index(rng.gen::<usize>() % possible_intersected_words.len());
                choice = Word {
                    position: temp.position,
                    direction: temp.direction,
                    word: temp.word.clone(),
                };
            } else if !possible_single_words.is_empty() {
                let temp = possible_single_words.index(rng.gen::<usize>() % possible_intersected_words.len());
               choice = Word {
                position: temp.position,
                direction: temp.direction,
                word: temp.word.clone(),
               }
            } else {
                eprintln!("Could not find a position for the word on the board!");
                return None;
            }

            let pos = choice.position;
            let dir = choice.direction;
            for (i, c) in current_word.chars().enumerate() {
                board.board.insert((pos.0 + i as i64 * dir.0, pos.1 + i as i64 * dir.1), c);
            }

            added_words.push(choice);
            words_by_len.remove(0);
        }  
        

        None
    }
}

fn validation(words: &[&str], size: i64) -> Option<Option<WordSearch>> {
    let chars_regex: Regex = Regex::new("[a-zA-Z]+").unwrap();
    let mut invalid_strings = Vec::<&str>::new();
    let mut long_strings = Vec::<&str>::new();
    for &s in words {
        if s.len() > size as usize {
            long_strings.push(s);
        }
        if chars_regex.is_match(s) == false {
            invalid_strings.push(s);
        }
    }
    if invalid_strings.is_empty() == false {
        eprintln!("Word list contained one or more invalid strings.");
        eprintln!("Invalid strings list:");
        for s in invalid_strings {
            eprintln!("{}", s);
        }
        return Some(None);
    }
    if long_strings.is_empty() == false {
        eprintln!("Word list contained one or more long strings (len < size).");
        eprintln!("Long strings list:");
        for s in long_strings {
            eprintln!("{}", s);
        }
        return Some(None);
    }
    None
}
