use std::{
    collections::{HashMap, HashSet},
    vec,
};

type DataType = Vec<Vec<char>>;
type NodeIdx = (usize, usize);

pub fn parse_input(input: &str) -> DataType {
    input
        .trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

type NodeGroup = (char, HashSet<NodeIdx>);
type GroupCollection = Vec<NodeGroup>;

pub fn find_groups(data: &DataType) -> GroupCollection {
    let mut group_collection = GroupCollection::new();
    let mut visited_nodes = HashSet::<(usize, usize)>::new();

    for (line_idx, line) in data.iter().enumerate() {
        for (column_idx, node_type) in line.iter().enumerate() {
            let node_idx = (line_idx, column_idx);

            if visited_nodes.contains(&node_idx) {
                continue;
            }

            let mut group = HashSet::<NodeIdx>::new();
            group.insert(node_idx);
            find_neighbours(data, node_idx, &mut group);

            for node in group.iter() {
                visited_nodes.insert(*node);
            }

            group_collection.push((*node_type, group));
        }
    }

    return group_collection;
}

fn get_neighbour_nodes_of_same_type(data: &DataType, node_idx: NodeIdx) -> Vec<NodeIdx> {
    let node_type = data[node_idx.0][node_idx.1];
    let available_positions = get_neighbour_nodes_of_all_types(data, node_idx);

    let mut nodes = Vec::<NodeIdx>::new();

    for p in available_positions {
        if data[p.0][p.1] == node_type {
            nodes.push(p);
        }
    }

    return nodes;
}

fn get_neighbour_nodes_of_all_types(data: &DataType, node_idx: NodeIdx) -> Vec<NodeIdx> {
    let mut available_positions = Vec::<NodeIdx>::new();

    if node_idx.0 > 0 {
        available_positions.push((node_idx.0 - 1, node_idx.1));
    }
    if node_idx.0 < data.len() - 1 {
        available_positions.push((node_idx.0 + 1, node_idx.1));
    }
    if node_idx.1 > 0 {
        available_positions.push((node_idx.0, node_idx.1 - 1));
    }
    if node_idx.1 < data[node_idx.1].len() - 1 {
        available_positions.push((node_idx.0, node_idx.1 + 1));
    }

    return available_positions;
}

fn find_neighbours(
    data: &DataType,
    field_idx: (usize, usize),
    seen_neighbours: &mut HashSet<NodeIdx>,
) {
    let neighbours = get_neighbour_nodes_of_same_type(data, field_idx);

    for neighbour in neighbours {
        if seen_neighbours.contains(&neighbour) {
            continue;
        }

        seen_neighbours.insert(neighbour);
        find_neighbours(data, neighbour, seen_neighbours);
    }
}

pub fn get_fence_cost(data: &DataType, group: &NodeGroup) -> usize {
    let area = group.1.len();
    let len: usize = group
        .1
        .iter()
        .map(|node| 4 - get_neighbour_nodes_of_same_type(data, *node).len())
        .sum();
    let cost = area * len;

    println!(
        "A region of {} plants with price = {} * {} = {}",
        group.0, area, len, cost
    );
    return cost;
}

pub fn get_fence_cost_with_discount(data: &DataType, group: &NodeGroup) -> usize {
    let area = group.1.len();

    let mut top_fences = Vec::<(NodeIdx, NodeIdx)>::new();
    let mut bottom_fences = Vec::<(NodeIdx, NodeIdx)>::new();

    let mut left_fences = Vec::<(NodeIdx, NodeIdx)>::new();
    let mut right_fences = Vec::<(NodeIdx, NodeIdx)>::new();

    for node in group.1.iter().cloned() {
        let neighbours = get_neighbour_nodes_of_same_type(data, node);
        if neighbours.len() == 4 {
            continue;
        }

        let has_top_fence = node.0 == 0 || !neighbours.contains(&(node.0 - 1, node.1));
        let has_bottom_fence = !neighbours.contains(&(node.0 + 1, node.1));
        let has_left_fence = node.1 == 0 || !neighbours.contains(&(node.0, node.1 - 1));
        let has_right_fence = !neighbours.contains(&(node.0, node.1 + 1));

        if has_top_fence {
            top_fences.push((node, (node.0, node.1 + 1)));
        }
        if has_left_fence {
            left_fences.push((node, (node.0 + 1, node.1)));
        }
        if has_bottom_fence {
            bottom_fences.push(((node.0 + 1, node.1), (node.0 + 1, node.1 + 1)));
        }
        if has_right_fence {
            right_fences.push(((node.0, node.1 + 1), (node.0 + 1, node.1 + 1)));
        }
    }

    let top_sections_count = reduce_sections_horizontal(&top_fences);
    let bottom_sections_count = reduce_sections_horizontal(&bottom_fences);

    let left_sections_count = reduce_sections_vertical(&left_fences);
    let right_sections_count = reduce_sections_vertical(&right_fences);

    assert_eq!(
        top_sections_count + bottom_sections_count,
        left_sections_count + right_sections_count
    );

    let fence_len =
        top_sections_count + bottom_sections_count + left_sections_count + right_sections_count;

    let cost = area * fence_len;

    println!(
        "A region of {} plants with price = {} * {} = {} \t\t [t: {}, b: {}, l: {}, r: {}]",
        group.0,
        area,
        fence_len,
        cost,
        top_sections_count,
        bottom_sections_count,
        left_sections_count,
        right_sections_count
    );
    return cost;
}

fn reduce_sections_1d(sections: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut sections = sections.clone();

    sections.sort_by(|(left_from, left_to), (right_from, right_to)| {
        if left_from == right_from {
            left_to.cmp(right_to)
        } else {
            left_to.cmp(right_to)
        }
    });

    'outer: loop {
        for i in 0..sections.len() - 1 {
            let left = sections.get(i).unwrap();
            let right = sections.get(i + 1).unwrap();
            if left.1 == right.0 {
                sections[i] = (left.0, right.1);
                sections.remove(i + 1);
                continue 'outer;
            }
        }
        break;
    }

    return sections;
}

fn reduce_sections_horizontal(sections: &Vec<((usize, usize), (usize, usize))>) -> usize {
    let mut one_d_section_map = HashMap::<usize, Vec<(usize, usize)>>::new();

    let only_horizontal_sections_iter = sections
        .iter()
        .filter(|((from_y, _), (to_y, _))| from_y == to_y);

    for ((from_y, from_x), (_, to_x)) in only_horizontal_sections_iter {
        let new_section = (*from_x, *to_x);
        one_d_section_map
            .entry(*from_y)
            .and_modify(|item| item.push(new_section))
            .or_insert(vec![new_section]);
    }

    let mut reduced_1d_map = HashMap::<usize, Vec<(usize, usize)>>::new();
    for (idx, sections) in one_d_section_map {
        let reduced_vec = reduce_sections_1d(&sections);
        reduced_1d_map.insert(idx, reduced_vec);
    }

    reduced_1d_map.values().map(|item| item.len()).sum()
}

fn reduce_sections_vertical(sections: &Vec<((usize, usize), (usize, usize))>) -> usize {
    let mut one_d_section_map = HashMap::<usize, Vec<(usize, usize)>>::new();

    let only_vertical_sections_iter = sections
        .iter()
        .filter(|((_, from_x), (_, to_x))| from_x == to_x);

    for ((from_y, from_x), (to_y, _)) in only_vertical_sections_iter {
        let new_section = (*from_y, *to_y);
        one_d_section_map
            .entry(*from_x)
            .and_modify(|item| item.push(new_section))
            .or_insert(vec![new_section]);
    }

    let mut reduced_1d_map = HashMap::<usize, Vec<(usize, usize)>>::new();
    for (idx, sections) in one_d_section_map {
        let reduced_vec = reduce_sections_1d(&sections);
        reduced_1d_map.insert(idx, reduced_vec);
    }

    reduced_1d_map.values().map(|item| item.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_sections_1d_test() {
        let data = vec![(1, 2), (5, 6), (2, 3)];
        let reduced = reduce_sections_1d(&data);
        assert_eq!(reduced, vec![(1, 3), (5, 6)]);
    }

    #[test]
    fn reduce_sections_horizontal_test() {
        let data = vec![((0, 0), (0, 1)), ((0, 1), (0, 2)), ((1, 1), (1, 2))];
        let reduced = reduce_sections_horizontal(&data);
        assert_eq!(reduced, 2);
    }

    #[test]
    fn reduce_sections_vertical_test() {
        let data = vec![((0, 0), (1, 0)), ((1, 0), (2, 0)), ((0, 1), (1, 1))];
        let reduced = reduce_sections_vertical(&data);
        assert_eq!(reduced, 2);
    }

    #[test]
    fn e2e_1() {
        let data = parse_input("EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE");
        let groups = find_groups(&data);
        let result: usize = groups
            .iter()
            .map(|group| get_fence_cost_with_discount(&data, group))
            .sum();
        assert_eq!(result, 236);
    }

    #[test]
    fn e2e_2() {
        let data = parse_input("AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA");
        let groups = find_groups(&data);
        let result: usize = groups
            .iter()
            .map(|group| get_fence_cost_with_discount(&data, group))
            .sum();
        assert_eq!(result, 368);
    }
}
