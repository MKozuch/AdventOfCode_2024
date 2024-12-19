use std::collections::{HashMap, HashSet};

pub type TowelMap = HashMap<usize, String>;
pub type TowelSet = HashSet<String>;
pub type DesignCache = HashMap<String, Option<Vec<usize>>>;
pub type DesignCache2 = HashMap<String, usize>;

pub fn parse_designs(input_designs: &str) -> Vec<String> {
    input_designs.lines().map(str::to_string).collect()
}

pub fn parse_towels(input_towels: &str) -> HashMap<usize, String> {
    let iter = input_towels
        .trim()
        .split(", ")
        .map(str::to_string)
        .enumerate();
    HashMap::from_iter(iter)
}

pub fn decompose_design(
    design: &str,
    towels: &TowelMap,
    cache: &mut DesignCache,
) -> Option<Vec<usize>> {
    //println!("Testing string {}", design);

    if cache.contains_key(design) {
        return cache.get(design).unwrap().to_owned();
    }

    if design.is_empty() {
        return Some(vec![]);
    }

    for (key, towel) in towels {
        if design.starts_with(towel) {
            let new_design = &design[towel.len()..];
            let decomposed = decompose_design(new_design, towels, cache);

            if let Some(mut vect) = decomposed {
                vect.push(*key);

                cache.insert(design.to_owned(), Some(vect.clone()));

                return Some(vect);
            }
        }
    }
    cache.insert(design.to_owned(), None);
    return None;
}

pub fn count_possible_decomps(
    design: &str,
    towels: &TowelSet,
    cache: &mut DesignCache2,
) -> usize {

    if design.is_empty() {
        return 1;
    }

    if cache.contains_key(design) {
        return cache[design];
    }

    let mut decomp_count = 0;

    for towel in towels {
        if design.starts_with(towel) {
            let new_design = &design[towel.len()..];
            decomp_count += count_possible_decomps(new_design, towels, cache);
        }
    }

    cache.insert(design.to_owned(), decomp_count);
    return decomp_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompose_design_test() {
        let design = "aaaa";
        let towels = HashMap::from_iter(vec!["a", "b"].iter().map(|s| s.to_string()).enumerate());
        let decomposed = decompose_design(design, &towels, &mut DesignCache::new());
        assert_eq!(decomposed.unwrap(), vec![0, 0, 0, 0]);

        let design = "ccc";
        let towels = HashMap::from_iter(vec!["a", "b"].iter().map(|s| s.to_string()).enumerate());
        let decomposed = decompose_design(design, &towels, &mut DesignCache::new());
        assert!(decomposed.is_none());

        let towels = parse_towels("r, wr, b, g, bwu, rb, gb, br");
        assert_eq!(
            decompose_design("brwrr", &towels, &mut DesignCache::new()).is_some(),
            true
        );
        assert_eq!(
            decompose_design("bggr", &towels, &mut DesignCache::new()).is_some(),
            true
        );
        assert_eq!(
            decompose_design("gbbr", &towels, &mut DesignCache::new()).is_some(),
            true
        );
        assert_eq!(
            decompose_design("rrbgbr", &towels, &mut DesignCache::new()).is_some(),
            true
        );
        assert_eq!(
            decompose_design("ubwu", &towels, &mut DesignCache::new()).is_some(),
            false
        );
        assert_eq!(
            decompose_design("bwurrg", &towels, &mut DesignCache::new()).is_some(),
            true
        );
        assert_eq!(
            decompose_design("brgr", &towels, &mut DesignCache::new()).is_some(),
            true
        );
        assert_eq!(
            decompose_design("bbrgwb", &towels, &mut DesignCache::new()).is_some(),
            false
        );
    }

    #[test]
    fn count_possible_decomps_test(){
        let towels = parse_towels("r, wr, b, g, bwu, rb, gb, br");
        let decomp_list: HashSet<String> = HashSet::from_iter(towels.values().cloned());
        let mut cache = HashMap::<String, usize>::new();
        
        let design = "brwrr";
        let decomp_count = count_possible_decomps(design, &decomp_list, &mut cache);
        assert_eq!(decomp_count, 2);
    }
}
