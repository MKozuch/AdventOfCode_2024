use std::{collections::HashMap, fs};

use day11::*;

fn main() {
    let input_path = "input/input.txt";

    let input = fs::read_to_string(input_path).unwrap();
    

    let mut stone_collection = parse_input(&input);
    for _ in 0..25 {
        stone_collection = blink_once(&stone_collection); // 185205
    }
    println!("Result: {}", stone_collection.len());

    let stone_collection = parse_input(&input);
    let mut cache = StoneCache::new();
    let mut ret: usize = 0;
    for stone in stone_collection.iter() {
        ret += count_stones_recursively(*stone, 75, &mut cache);
    }
    println!("Result 2: {}", ret);
}
