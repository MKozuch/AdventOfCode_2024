use regex::{Regex, RegexBuilder};

pub fn parse_input(input: &str) -> Vec<(i64, i64)> {

    let re = Regex::new(r"(?:mul\((\d+,\d+)\))").unwrap();

    re.captures_iter(input)
        .map(|captures| captures.get(1).unwrap().as_str())
        .map(|str| str.split_once(",").unwrap())
        .map(|str_pair| (str_pair.0.parse::<i64>().unwrap(), str_pair.1.parse::<i64>().unwrap()))
        .collect()
}

pub fn calc_multiplications(pairs: &Vec<(i64, i64)>) -> i64 {

    pairs.iter()
        .map(|pair| pair.0 * pair.1)
        .sum()
}

pub fn parse_input_2(input: &str) -> Vec<(i64, i64)> {

    let re = RegexBuilder::new(r"(?:^|do\(\))(.*?)(?:don't\(\)|$)")
        .multi_line(false)
        .dot_matches_new_line(true)
        .build().unwrap();

    re.captures_iter(input)
        .map(|captures| captures.get(1).unwrap().as_str())
        .map(|str| parse_input(str))
        .flatten()
        .collect()
}
