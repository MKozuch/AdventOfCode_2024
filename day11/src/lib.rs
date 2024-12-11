use std::{collections::HashMap, u64, usize};

pub type Stone = u64;
pub type StoneCollection = Vec<Stone>;
pub type StoneCache = HashMap<(Stone, usize), usize>;

pub fn parse_input(input: &str) -> StoneCollection {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|item| item.parse::<Stone>().unwrap())
        .collect()
}

fn count_digits(num: Stone) -> u64 {
    (num as f64).log10().floor() as u64 + 1
}

fn push_stone(stone_collection: &mut StoneCollection, stone: Stone) {
    if stone == 0 {
        stone_collection.push(1);
        return;
    }

    let digit_count = count_digits(stone) as u32;

    if digit_count % 2 == 0 {
        let div = 10u64.pow(digit_count / 2);
        let left = stone / div;
        let right = stone % div;
        stone_collection.push(left);
        stone_collection.push(right);
        return;
    }

    stone_collection.push(stone * 2024);
}

pub fn blink_once(stone_collection: &StoneCollection) -> StoneCollection {
    let mut new_stone_collection = StoneCollection::with_capacity(stone_collection.len() * 2);

    for stone in stone_collection {
        push_stone(&mut new_stone_collection, *stone);
    }
    return new_stone_collection;
}

pub fn count_stones_recursively(stone: Stone, depth: usize, cache: &mut StoneCache) -> usize {
    if cache.contains_key(&(stone, depth)) {
        return cache.get(&(stone, depth)).unwrap().to_owned();
    }

    if depth == 0 {
        // print!("{stone} ");
        return 1;
    }

    let next_depth = depth - 1;

    if stone == 0 {
        let result = count_stones_recursively(1, next_depth, cache);
        cache.insert((stone, depth), result);
        return result;
    }

    let result: usize;
    let digit_count = count_digits(stone) as u32;

    if digit_count % 2 == 0 {
        let div = 10u64.pow(digit_count / 2);
        let left_stone = stone / div;
        let right_stone = stone % div;
        result = count_stones_recursively(left_stone, next_depth, cache)
            + count_stones_recursively(right_stone, next_depth, cache);
    } else {
        result = count_stones_recursively(stone * 2024, next_depth, cache);
    }

    cache.insert((stone, depth), result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_stone_test() {
        let mut stone_collection = StoneCollection::new();
        for stone in [0, 1, 10, 99, 999].iter() {
            push_stone(&mut stone_collection, *stone);
        }
        assert_eq!(stone_collection, [1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn recursive_test() {
        let mut cache = StoneCache::new();

        assert_eq!(count_stones_recursively(1, 1, &mut cache), 1,);
        println!();
        assert_eq!(count_stones_recursively(100, 1, &mut cache), 1);
        println!();
        assert_eq!(count_stones_recursively(1000, 1, &mut cache), 2);
        println!();
        assert_eq!(count_stones_recursively(1000, 2, &mut cache), 3);
        println!();
        assert_eq!(count_stones_recursively(100, 2, &mut cache), 2);
        println!();
        assert_eq!(count_stones_recursively(100, 3, &mut cache), 2);
        println!();
        assert_eq!(count_stones_recursively(100, 4, &mut cache), 4);
        println!();
    }

    #[test]
    fn recursive_test_with_collection() {
        let stone_collection: Vec<Stone> = vec![0, 1, 10, 99, 999];
        let mut cache = StoneCache::new();

        let mut res: usize = 0;
        for stone in stone_collection.iter() {
            res += count_stones_recursively(*stone, 1, &mut cache);
        }
        assert_eq!(res, 7);
        println!();

        let stone_collection: Vec<Stone> = vec![125, 17];
        let mut res: usize = 0;
        for stone in stone_collection.iter() {
            res += count_stones_recursively(*stone, 6, &mut cache);
        }
        assert_eq!(res, 22);
        println!()
    }

    #[test]
    fn it_works() {
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(9), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(99), 2);
        assert_eq!(count_digits(100), 3);
    }
}
