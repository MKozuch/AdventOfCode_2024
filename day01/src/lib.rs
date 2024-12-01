use std::iter::zip;
use std::collections::HashMap;

pub fn parse_input(input: String) -> (Vec<i64>, Vec<i64>){

    let int_pairs = input.trim().split('\n')
        .map(|str| str.trim())
        .map(|str| str.split_once(char::is_whitespace).unwrap())
        .map(|pair| (pair.0.trim().parse::<i64>().unwrap(), pair.1.trim().parse::<i64>().unwrap()));

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for pair in int_pairs{
        left.push(pair.0);
        right.push(pair.1);
    }

    (left, right)
}

pub fn calc_dist(left: &Vec<i64>, right: &Vec<i64>) -> i64{

    zip(left, right)
        .map(|pair| (pair.0 - pair.1).abs())
        .sum()
}

pub fn count_entries(vector: &Vec<i64>) -> HashMap<i64, i64>{

    let mut entry_count = HashMap::<i64, i64>::new();

    for number in vector {
        entry_count.entry(*number).and_modify(|count| *count += 1).or_insert(1);
    }

    entry_count
}

pub fn calc_similarity_score(left: &Vec<i64>, right: &Vec<i64>) -> i64{

    let counts = count_entries(right);

    left.iter()
        .map(|number| counts.get(number).unwrap_or(&0) * number)
        .sum()
}