use std::{cmp::min, collections::HashMap, usize};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Pos {
    pub y: i64,
    pub x: i64,
}

pub fn get_map_size(input: &str) -> (usize, usize) {
    let height = input.chars().filter(|c| *c == '\n').count() + 1;
    let width = input.split_once('\n').unwrap().0.trim().len();
    (height, width)
}

fn calc_nodes_for_pair(pos1: &Pos, pos2: &Pos) -> (Pos, Pos) {
    let dy = pos2.y - pos1.y;
    let dx = pos2.x - pos1.x;

    (
        Pos {
            y: pos1.y - dy,
            x: pos1.x - dx,
        },
        Pos {
            y: pos2.y + dy,
            x: pos2.x + dx,
        },
    )
}

fn calc_nodes_for_pair_in_bounds(
    pos1: &Pos,
    pos2: &Pos,
    map_height: usize,
    map_width: usize,
) -> Vec<Pos> {
    let mut dy = pos2.y - pos1.y;
    let mut dx = pos2.x - pos1.x;

    for i in 2..=min(dx.abs(), dy.abs()) {
        if dx % i == 0 && dy % i == 0 {
            dx /= i;
            dy /= i;
        }
    }

    let mut nodes = Vec::<Pos>::new();

    let mut i = 0;
    loop {
        let new_pos = Pos {
            y: pos1.y + dy * i,
            x: pos1.x + dx * i,
        };

        if is_valid_pos(&new_pos, map_height, map_width) {
            nodes.push(new_pos);
            i += 1;
            continue;
        }

        break;
    }

    let mut i = -1;
    loop {
        let new_pos = Pos {
            y: pos1.y + dy * i,
            x: pos1.x + dx * i,
        };

        if is_valid_pos(&new_pos, map_height, map_width) {
            nodes.push(new_pos);
            i -= 1;
            continue;
        }

        break;
    }

    return nodes;
}

pub fn calc_node_positions_for_antenna_type(positions: &[Pos]) -> Vec<Pos> {
    let mut nodes = Vec::<Pos>::new();

    for i in 0..positions.len() - 1 {
        let pos1 = positions.get(i).unwrap();

        for j in i + 1..positions.len() {
            let pos2 = positions.get(j).unwrap();

            let (node1, node2) = calc_nodes_for_pair(pos1, pos2);
            nodes.push(node1);
            nodes.push(node2);
        }
    }

    return nodes;
}

pub fn calc_node_positions_for_antenna_type_2(
    positions: &[Pos],
    map_height: usize,
    map_width: usize,
) -> Vec<Pos> {
    let mut nodes = Vec::<Pos>::new();

    for i in 0..positions.len() - 1 {
        let pos1 = positions.get(i).unwrap();

        for j in i + 1..positions.len() {
            let pos2 = positions.get(j).unwrap();

            nodes.append(&mut calc_nodes_for_pair_in_bounds(
                pos1, pos2, map_height, map_width,
            ));
        }
    }

    return nodes;
}

pub fn parse_input(input: &str) -> HashMap<char, Vec<Pos>> {
    let mut map = HashMap::<char, Vec<Pos>>::new();

    let lines = input.trim().split("\n").enumerate();

    for (y, line) in lines {
        for (x, item) in line.char_indices() {
            if item == '.' {
                continue;
            }

            let entry = map.entry(item).or_insert(vec![]);
            entry.push(Pos {
                y: y as i64,
                x: x as i64,
            });
        }
    }

    return map;
}

pub fn is_valid_pos(pos: &Pos, map_height: usize, map_width: usize) -> bool {
    pos.x >= 0 && pos.y >= 0 && (pos.x as usize) < map_width && (pos.y as usize) < map_height
}

#[test]
fn test_calc_nodes_for_pair() {
    let pos1 = Pos { y: 3, x: 4 };
    let pos2 = Pos { y: 5, x: 5 };
    let (node1, node2) = calc_nodes_for_pair(&pos1, &pos2);

    let exp_node1 = Pos { y: 1, x: 3 };
    let exp_node2 = Pos { y: 7, x: 6 };

    assert_eq!(node1, exp_node1);
    assert_eq!(node2, exp_node2);
}
