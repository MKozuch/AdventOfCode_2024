use std::collections::HashMap;

pub fn parse_rules(rules_input: &str) -> Vec<(i64, i64)> {
    rules_input
        .trim()
        .split("\n")
        .map(|line| line.split_once("|").unwrap())
        .map(|pair| {
            (
                pair.0.parse::<i64>().unwrap(),
                pair.1.parse::<i64>().unwrap(),
            )
        })
        .collect()
}

pub fn parse_pages(pages_input: &str) -> Vec<Vec<i64>> {
    pages_input
        .trim()
        .split("\n")
        .map(|line| line.split(",").map(|i| i.parse::<i64>().unwrap()).collect())
        .collect()
}

pub fn rules_to_key(rules: &Vec<(i64, i64)>) -> HashMap<i64, i64> {
    let mut all_pages: Vec<i64> = rules
        .iter()
        .map(|pair| vec![pair.0, pair.1])
        .flatten()
        .collect();
    all_pages.sort();
    all_pages.dedup();

    let mut changed = true;

    while changed {
        changed = false;

        for rule in rules {
            let left_idx = all_pages.iter().position(|item| item == &rule.0).unwrap();
            let right_idx = all_pages.iter().position(|item| item == &rule.1).unwrap();

            if left_idx > right_idx {
                all_pages.swap(left_idx, right_idx);
                changed = true;
            }
        }
    }

    HashMap::<i64, i64>::from_iter(
        all_pages
            .iter()
            .enumerate()
            .map(|pair| (*pair.1 as i64, pair.0 as i64)),
    )
}

pub fn is_order_valid(pages: &Vec<i64>, rules: &Vec<(i64, i64)>) -> bool {
    for rule in rules {
        let left_idx = pages.iter().position(|item| item == &rule.0);
        let right_idx = pages.iter().position(|item| item == &rule.1);

        if left_idx.is_none() || right_idx.is_none() {
            continue;
        };

        if left_idx.unwrap() > right_idx.unwrap() {
            return false;
        }
    }

    return true;
}

pub fn is_order_valid_2(pages: &Vec<i64>, key: &HashMap<i64, i64>) -> bool {
    let mut sorted = pages.clone();
    sorted.sort_by_key(|item| key[item]);
    return &sorted == pages;
}

pub fn get_middle_page(pages: &Vec<i64>) -> i64 {
    let mid_index = (pages.len() - 1) / 2;
    return pages[mid_index];
}

pub fn fix_sorting(page: &Vec<i64>, rules: &Vec<(i64, i64)>) -> Vec<i64> {
    let mut new_page = page.clone();
    let mut changed = true;

    while changed {
        changed = false;

        for rule in rules {
            let left_idx_opt = new_page.iter().position(|item| item == &rule.0);
            let right_idx_opt = new_page.iter().position(|item| item == &rule.1);

            if left_idx_opt.is_some() && right_idx_opt.is_some() {
                let left_idx = left_idx_opt.unwrap();
                let right_idx = right_idx_opt.unwrap();

                if left_idx > right_idx {
                    new_page.swap(left_idx, right_idx);
                    changed = true;
                    // println!("Swapping elements {left_idx} and {right_idx} according to rule ({}|{})", rule.0, rule.1);
                }
            }
        }
    }
    return new_page;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rules() {
        let input = "1|2\n3|4";
        let expected = vec![(1, 2), (3, 4)];
        assert_eq!(parse_rules(input), expected);
    }

    #[test]
    fn test_parse_pages() {
        let input = "1,2,3\n4,5,6";
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(parse_pages(input), expected);
    }

    #[test]
    fn test_is_order_valid() {
        let pages = vec![1, 2, 3, 4];
        let rules = vec![(1, 2), (2, 3), (3, 4)];
        let key = rules_to_key(&rules);
        assert!(is_order_valid_2(&pages, &key));

        let pages = vec![1, 3, 2, 4];
        let key = rules_to_key(&rules);
        assert!(!is_order_valid_2(&pages, &key));
    }

    #[test]
    fn test_get_middle_page() {
        let pages = vec![1, 2, 3, 4, 5];
        assert_eq!(get_middle_page(&pages), 3);
    }

    #[test]
    fn test_rules_to_key() {
        let rules = vec![(10, 20), (20, 30), (30, 40), (10, 20)];
        let key = rules_to_key(&rules);
        let expected: HashMap<i64, i64> = [(10, 0), (20, 1), (30, 2), (40, 3)]
            .iter()
            .cloned()
            .collect();
        assert_eq!(key, expected);
    }

    #[test]
    fn test_rules_to_key_with_unsorted_rules() {
        let rules = vec![(30, 40), (10, 20), (20, 30)];
        let key = rules_to_key(&rules);
        let expected: HashMap<i64, i64> = [(10, 0), (20, 1), (30, 2), (40, 3)]
            .iter()
            .cloned()
            .collect();
        assert_eq!(key, expected);
    }

    #[test]
    fn test_rules_to_key_with_implicit_order() {
        let rules = vec![(20, 30), (10, 20)];
        let key = rules_to_key(&rules);
        let expected: HashMap<i64, i64> = [(10, 0), (20, 1), (30, 2)].iter().cloned().collect();
        assert_eq!(key, expected);
    }

    #[test]
    fn test_rules_to_key_sorted() {
        let range_max = 100;

        let range: Vec<i64> = (0..range_max).collect();
        let range2: Vec<i64> = (0..range_max).collect();

        let rules = range
            .iter()
            .zip(range2.iter())
            .map(|pair| (*pair.0, *pair.1))
            .collect();
        let key = rules_to_key(&rules);

        let expected = HashMap::from_iter(rules.iter().map(|pair| (pair.0, pair.1)));
        assert_eq!(key, expected);
    }

    #[test]
    fn test_rules_to_key_sorted_rev() {
        let range_max = 100;

        let range: Vec<i64> = (0..range_max).collect();
        let range2: Vec<i64> = (0..range_max).collect();

        let rules = range
            .iter()
            .zip(range2.iter())
            .map(|pair| (*pair.0, *pair.1))
            .rev()
            .collect();
        let key = rules_to_key(&rules);

        let expected = HashMap::from_iter(rules.iter().map(|pair| (pair.0, pair.1)));
        assert_eq!(key, expected);
    }
}
