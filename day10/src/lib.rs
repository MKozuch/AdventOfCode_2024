use std::char;

pub type Pos = (usize, usize);
pub type Trail = Vec<Pos>;
pub type TopoMap = Vec<Vec<u8>>;

fn map_width(topo_map: &TopoMap) -> usize {
    return topo_map[0].len();
}

fn map_height(topo_map: &TopoMap) -> usize {
    return topo_map.len();
}

pub fn parse_input(input: &str) -> TopoMap {
    let parse_line = |line: &str| {
        line.chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect()
    };

    input.trim().split("\n").map(parse_line).collect()
}

pub fn find_start_points(topo_map: &TopoMap) -> Vec<Pos> {
    let mut start_points = Vec::<Pos>::new();

    for (line_idx, line) in topo_map.iter().enumerate() {
        for (column_idx, val) in line.iter().enumerate() {
            if *val == 0 {
                start_points.push((line_idx, column_idx));
            }
        }
    }

    return start_points;
}

pub fn find_trails_from(topo_map: &TopoMap, start_position: &Pos) -> Vec<Trail> {
    let current_elevation = topo_map[start_position.0][start_position.1];

    if current_elevation == 9 {
        let trail = vec![start_position.to_owned()];
        return vec![trail];
    }

    let mut possible_dirs = Vec::<(i8, i8)>::new();
    if start_position.0 > 0 {
        possible_dirs.push((-1, 0));
    }
    if start_position.1 > 0 {
        possible_dirs.push((0, -1));
    }
    if start_position.0 < map_height(topo_map) - 1 {
        possible_dirs.push((1, 0));
    }
    if start_position.1 < map_width(topo_map) - 1 {
        possible_dirs.push((0, 1));
    }

    let mut trails = Vec::<Trail>::new();

    for dir in possible_dirs {
        let new_pos = (
            (start_position.0 as isize + dir.0 as isize) as usize,
            (start_position.1 as isize + dir.1 as isize) as usize,
        );
        let new_elevation = topo_map[new_pos.0][new_pos.1];
        if new_elevation > current_elevation && new_elevation - current_elevation == 1 {
            for mut new_trail in find_trails_from(topo_map, &new_pos) {
                new_trail.insert(0, *start_position);
                trails.push(new_trail);
            }
        }
    }

    return trails;
}

pub fn draw_trail(topo_map: &TopoMap, trail: &Trail) {
    let map_height = map_height(topo_map);
    let map_width = map_width(topo_map);

    let line = Vec::<char>::from_iter((0..map_width).map(|_| '.'));

    let mut lines = Vec::<Vec<char>>::from_iter((0..map_height).map(|_| line.clone()));

    for (idx, pos) in trail.iter().enumerate() {
        let idx = idx as u32;
        lines[pos.0][pos.1] = char::from_digit(idx, 10).unwrap();
    }

    for line in lines {
        println!("{}", String::from_iter(line));
    }
    println!("");
}
