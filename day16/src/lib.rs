#[derive(PartialEq)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(PartialEq, Clone, Copy)]
pub enum TraverseAction {
    Forward,
    TurnLeft,
    TurnRight,
}

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn get_start_pos(labyrinth: &Vec<Vec<char>>) -> (usize, usize) {
    (labyrinth.len() - 2, 1)
}

pub fn get_end_pos(labyrinth: &Vec<Vec<char>>) -> (usize, usize) {
    (1, labyrinth[1].len() - 2)
}

pub type Labyrinth = Vec<Vec<char>>;
pub type ActionHistory = Vec<TraverseAction>;
pub type PositionHistory = Vec<(usize, usize)>;

pub fn get_next_actions(
    labyrinth: &Labyrinth,
    action_history: &ActionHistory,
    fields_history: &PositionHistory,
) -> Vec<(Vec<TraverseAction>, (usize, usize))> {
    let left_turns = action_history
        .iter()
        .filter(|item| **item == TraverseAction::TurnLeft)
        .count();
    let right_turns = action_history
        .iter()
        .filter(|item| **item == TraverseAction::TurnRight)
        .count();
    let diff = (((right_turns - left_turns) % 4) + 4) % 4;
    let current_direction = match diff {
        0 => Direction::Up,
        1 => Direction::Right,
        2 => Direction::Down,
        3 => Direction::Left,
        _ => panic!(),
    };
    let current_pos = fields_history.last().unwrap();

    let to_north = (current_pos.0 - 1, current_pos.1);
    let to_east = (current_pos.0, current_pos.1 + 1);
    let to_south = (current_pos.0 + 1, current_pos.1);
    let to_west = (current_pos.0, current_pos.1 - 1);

    let to_front;
    let to_left;
    let to_right;

    match current_direction {
        Direction::Up => {
            to_front = to_north;
            to_left = to_west;
            to_right = to_east;
        }
        Direction::Right => {
            to_front = to_east;
            to_left = to_north;
            to_right = to_south;
        }
        Direction::Down => {
            to_front = to_south;
            to_left = to_east;
            to_right = to_west;
        }
        Direction::Left => {
            to_front = to_west;
            to_left = to_south;
            to_right = to_north;
        }
    };

    let mut possible_actions = Vec::<(Vec<TraverseAction>, (usize, usize))>::new();

    // check in front
    let in_front = labyrinth[to_front.0][to_front.1];
    if in_front != '#' && !fields_history.contains(&to_front) {
        possible_actions.push((vec![TraverseAction::Forward], (to_front)));
    }

    let last_action = action_history.last();

    if last_action.is_none_or(|item| *item == TraverseAction::Forward) {
        let in_left = labyrinth[to_left.0][to_left.1];
        if in_left != '#' && !fields_history.contains(&to_left) {
            possible_actions.push((
                vec![TraverseAction::TurnLeft, TraverseAction::Forward],
                (to_left),
            ));
        }

        let in_right = labyrinth[to_right.0][to_right.1];
        if in_right != '#' && !fields_history.contains(&to_right) {
            possible_actions.push((
                vec![TraverseAction::TurnRight, TraverseAction::Forward],
                (to_right),
            ));
        }
    }

    return possible_actions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data;
        let result = do_calculations(&data);
        let exp_result;
        assert_eq!(result, exp_result);
    }
}
