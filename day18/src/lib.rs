use std::{rc::Rc, usize};

pub type Pos = (u64, u64);

#[derive(Clone)]
pub struct Node {
    pos: Pos,
    parent: Option<Rc<Node>>,
}

pub fn parse_input(input: &str, len: usize) -> Vec<Pos> {
    let parse_line = |line: &str| {
        let (left, right) = line.split_once(',').unwrap();
        (right.parse::<u64>().unwrap(), left.parse::<u64>().unwrap())
    };

    input
        .trim()
        .lines()
        .enumerate()
        .filter(|(idx, _)| *idx < len)
        .map(|(_, line)| parse_line(line))
        .collect()
}

pub fn path_cost(node: &Node) -> u64 {
    if node.parent.is_none() {
        return 0;
    }

    return 1 + path_cost(node.parent.as_ref().unwrap());
}

pub fn heuristic(node: &Node, target: Pos) -> u64 {
    let node_pos = node.pos;
    node_pos.0.abs_diff(target.0) + node_pos.0.abs_diff(target.0)
}

pub fn total_estimated_cost(node: &Node, target: Pos) -> u64 {
    path_cost(node) + heuristic(node, target)
}

pub fn find_path(
    grid_size: u64,
    corrupted_blocks: &[Pos],
    start_point: Pos,
    end_point: Pos,
) -> Option<Node> {
    let root = Node {
        pos: start_point,
        parent: None,
    };

    let mut open_list: Vec<Node> = vec![root];
    let mut closed_list: Vec<Pos> = vec![];

    while !open_list.is_empty() {
        open_list.sort_by_key(|node| total_estimated_cost(node, end_point) as i64 * -1);

        let current_node = open_list.pop().unwrap();
        let current_cost = path_cost(&current_node);

        if current_node.pos == end_point {
            return Some(current_node);
        }

        let new_neighbours_pos = get_neighbours(&current_node, grid_size);
        for neighbour_pos in new_neighbours_pos.iter() {
            if corrupted_blocks.contains(&neighbour_pos) {
                continue;
            }
            if closed_list.contains(&neighbour_pos) {
                continue;
            }

            let neighbour_node = Node {
                pos: *neighbour_pos,
                parent: Some(Rc::new(current_node.clone())),
            };

            let index = open_list
                .iter()
                .position(|node| node.pos == neighbour_node.pos);
            if let Some(index) = index {
                // checking if we found a better way to reach this node
                let other_node = open_list.get_mut(index).unwrap();
                let other_cost = path_cost(other_node);
                if (current_cost + 1) < other_cost {
                    other_node.parent = Some(Rc::new(current_node.clone()));
                }
            } else {
                open_list.push(neighbour_node);
            }
        }

        closed_list.push(current_node.pos);
    }
    return None;
}

fn get_neighbours(node: &Node, grid_size: u64) -> Vec<Pos> {
    let mut neighbours: Vec<Pos> = vec![];
    let node_pos = node.pos;

    if node.pos.0 > 0 {
        // top
        neighbours.push((node_pos.0 - 1, node_pos.1));
    }
    if node.pos.0 < grid_size - 1 {
        // bottom
        neighbours.push((node_pos.0 + 1, node_pos.1));
    }
    if node.pos.1 > 0 {
        // left
        neighbours.push((node_pos.0, node_pos.1 - 1));
    }
    if node.pos.1 < grid_size - 1 {
        // right
        neighbours.push((node_pos.0, node_pos.1 + 1));
    }

    return neighbours;
}

pub fn unroll_path(node: &Node) -> Vec<Pos> {
    if node.parent.is_none() {
        vec![node.pos]
    }
    else {
        let mut vec = unroll_path(node.parent.as_ref().unwrap().as_ref());
        vec.push(node.pos);
        return vec;
    }

}

pub fn draw_grid(grid_size: u64, corrupted_blocks: &[Pos], path: &Vec<Pos>){
    for line_idx in 0..grid_size {
        for column_idx in 0..grid_size {
            if corrupted_blocks.contains(&(line_idx, column_idx)){
                print!("{}", '#');
            }
            else if path.contains(&(line_idx, column_idx)){
                print!("{}", 'O');
            }
            else {
                print!("{}", '.');
            }
        }
        print!("\n");
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let root = Rc::new(Node {
            pos: (0, 0),
            parent: None,
        });
        let node_1 = Rc::new(Node {
            pos: (0, 0),
            parent: Some(Rc::clone(&root)),
        });
        let node_2 = Rc::new(Node {
            pos: (0, 0),
            parent: Some(Rc::clone(&node_1)),
        });

        assert_eq!(path_cost(root.as_ref()), 0);
        assert_eq!(path_cost(node_1.as_ref()), 1);
        assert_eq!(path_cost(node_2.as_ref()), 2);
    }
}
