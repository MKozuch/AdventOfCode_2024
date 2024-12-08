use day08::*;
use std::fs;

fn main() {
    let input_path = "input/input.txt";
    //let input_path = "input/test_input.txt";

    let input = fs::read_to_string(input_path).unwrap();

    let antennas_map = parse_input(&input);

    let (map_height, map_width) = get_map_size(&input);

    let nodes_iter = antennas_map
        .values()
        .map(|antenna_positions| calc_node_positions_for_antenna_type(antenna_positions));

    let node_list: Vec<Pos> = nodes_iter.flatten().collect();

    let mut valid_nodes: Vec<Pos> = node_list
        .iter()
        .filter(|node_pos| is_valid_pos(node_pos, map_height, map_width))
        .copied()
        .collect();

    valid_nodes.sort();

    valid_nodes.dedup();

    println!("Node count: {}", valid_nodes.len());

    // p2
    let nodes_iter = antennas_map.values().map(|antenna_positions| {
        calc_node_positions_for_antenna_type_2(antenna_positions, map_height, map_width)
    });

    let node_list: Vec<Pos> = nodes_iter.flatten().collect();

    let mut valid_nodes: Vec<Pos> = node_list
        .iter()
        .filter(|node_pos| is_valid_pos(node_pos, map_height, map_width))
        .copied()
        .collect();

    valid_nodes.sort();

    valid_nodes.dedup();

    println!("Node count 2: {}", valid_nodes.len());
}
