use std::{iter::zip, usize};

pub fn unroll_input(input: &str) -> Vec<Option<u32>> {
    let mut out = Vec::<Option<u32>>::with_capacity(input.len());

    let mut is_file = true;
    let mut id_counter: u32 = 0;

    for chr in input.chars() {
        let size: u32 = chr.to_string().parse().unwrap();

        if is_file {
            for _ in 0..size {
                out.push(Some(id_counter));
            }
            id_counter += 1;
        } else {
            for _ in 0..size {
                out.push(None);
            }
        }

        is_file ^= true;
    }

    return out;
}

pub fn mem_to_str(vec: &Vec<Option<u32>>) -> String {
    vec.iter()
        .map(|item| match item {
            Some(i) => i.to_string(),
            None => ".".to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn rearrange_simple(mem: &[Option<u32>]) -> Vec<Option<u32>> {
    let mut out = mem.to_owned();

    let forward_indices: Vec<usize> = out
        .iter()
        .enumerate()
        .filter_map(|(i, item)| if item.is_none() { Some(i) } else { None })
        .collect();

    let reverse_indices: Vec<usize> = out
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(i, item)| if item.is_some() { Some(i) } else { None })
        .collect();

    for (left_idx, right_idx) in zip(forward_indices, reverse_indices) {
        if left_idx < right_idx {
            out.swap(left_idx, right_idx);
        } else {
            break;
        }
    }

    return out;
}

pub fn calc_checksum(mem: &[Option<u32>]) -> u64 {
    mem
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, val)| match val {
            Some(v) => acc + idx as u64 * (*v as u64),
            None => acc,
        })
}

fn find_free_span(mem: &[Option<u32>], size: usize) -> Option<(usize, usize)> {
    let mut i = 0;

    'outer: loop {
        while i < mem.len() && mem[i].is_some() {
            i += 1;

            if i == mem.len() {
                break 'outer;
            }
        }

        let section_begin = i;

        while i < mem.len() && mem[i].is_none() {
            i += 1;
        }

        let section_len = i - section_begin;

        if section_len >= size {
            return Some((section_begin, section_len));
        }

        if i == mem.len() {
            break 'outer;
        }
    }

    return None;
}

fn find_file_span(mem: &[Option<u32>], file_id: u32) -> Option<(usize, usize)> {
    let start_idx_opt = mem
        .iter()
        .position(|item| item.is_some() && item.unwrap() == file_id);

    if start_idx_opt.is_none() {
        return None;
    }

    let start_idx = start_idx_opt.unwrap();

    let mut span_len = 0;
    while start_idx + span_len < mem.len() && mem[start_idx + span_len] == Some(file_id) {
        span_len += 1;
    }

    return Some((start_idx, span_len));
}

pub fn rearrange_smart(mem: &[Option<u32>]) -> Vec<Option<u32>> {
    let mut out = mem.to_owned();

    let max_block_id = mem
        .iter()
        .rev()
        .find(|item| item.is_some())
        .unwrap()
        .unwrap();

    for file_id in (0..=max_block_id).rev() {
        // println!("{}", to_str(&out));

        let (file_offset, file_size) = find_file_span(mem, file_id).unwrap();
        let span_opt = find_free_span(&out, file_size);
        if span_opt.is_none() {
            continue;
        }

        let (free_offset, _) = span_opt.unwrap();

        if free_offset > file_offset {
            continue;
        }

        for (a, b) in zip(
            file_offset..file_offset + file_size,
            free_offset..free_offset + file_size,
        ) {
            out.swap(a, b);
        }
    }

    return out;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let input = "2333133121414131402";

        let unrolled = unroll_input(input);
        let unrolled_str = mem_to_str(&unrolled);
        let unrolled_str_exp = "00...111...2...333.44.5555.6666.777.888899";
        assert_eq!(unrolled_str_exp, unrolled_str);

        let rearranged = rearrange_simple(&unrolled);
        let rearranged_str = mem_to_str(&rearranged);
        let rearranged_str_exp = "0099811188827773336446555566..............";
        assert_eq!(rearranged_str_exp, rearranged_str);

        let checksum = calc_checksum(&rearranged);
        let checksum_exp = 1928;
        assert_eq!(checksum_exp, checksum);

        let rearranged_smart = rearrange_smart(&unrolled);
        let rearranged_smart_str = mem_to_str(&rearranged_smart);
        let rearranged_smart_str_exp = "00992111777.44.333....5555.6666.....8888..";
        assert_eq!(rearranged_smart_str_exp, rearranged_smart_str);
    }

    #[test]
    fn find_span_test() {
        let input = "214";
        let unrolled = unroll_input(input);
        let span = find_file_span(&unrolled, 1);
        assert_eq!((3, 4), span.unwrap());
    }
}
