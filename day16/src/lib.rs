use std::path;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

impl Direction {
    pub fn to_vec(&self) -> (isize, isize) {
        match self {
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::North => (-1, 0),
        }
    }
}

impl From<isize> for Direction {
    fn from(value: isize) -> Self {
        let value = value.rem_euclid(4);
        match value {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::North,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum TraverseAction {
    Forward = 0,
    TurnLeft = -1,
    TurnRight = 1,
}

impl From<isize> for TraverseAction {
    fn from(value: isize) -> Self {
        match value {
            0 => TraverseAction::Forward,
            1 => TraverseAction::TurnRight,
            3 => TraverseAction::TurnLeft,
            _ => unreachable!(),
        }
    }
}

impl TraverseAction {
    fn cost(&self) -> usize {
        match self {
            TraverseAction::Forward => 1,
            TraverseAction::TurnLeft => 1001,
            TraverseAction::TurnRight => 1001,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

pub fn get_start_pos(labyrinth: &Vec<Vec<char>>) -> (usize, usize) {
    (labyrinth.len() - 2, 1)
}

pub fn get_end_pos(labyrinth: &Vec<Vec<char>>) -> (usize, usize) {
    (1, labyrinth[1].len() - 2)
}

pub type Labyrinth = Vec<Vec<char>>;
pub type ActionHistory = Vec<TraverseAction>;
pub type Position = (usize, usize);
pub type PositionHistory = Vec<Position>;

pub fn draw_labyrinth_and_path(labyrinth: &Labyrinth, pos_history: &PositionHistory) {
    let mut labyrinth = labyrinth.clone();

    for pos in pos_history {
        labyrinth[pos.0][pos.1] = 'O';
    }

    for row in labyrinth {
        let str = String::from_iter(row.iter());
        println!("{}", str);
    }
}

pub fn draw_labyrinth_and_path_2(labyrinth: &Labyrinth, path: &Path) {
    let mut labyrinth = labyrinth.clone();

    for (pos, action) in path.pos_history.iter().zip(path.action_history.iter()) {
        let chr = match action {
            TraverseAction::Forward => 'O',
            TraverseAction::TurnLeft => 'L',
            TraverseAction::TurnRight => 'R',
        };

        labyrinth[pos.0][pos.1] = chr;
    }

    for row in labyrinth {
        let str = String::from_iter(row.iter());
        println!("{}", str);
    }
}

#[derive(Clone)]
pub struct Path {
    pos_history: PositionHistory,
    action_history: ActionHistory,
    cost: usize,
}

impl Path {
    pub fn new(init_pos: Position) -> Path {
        Path {
            pos_history: vec![init_pos],
            action_history: vec![],
            cost: 0,
        }
    }

    pub fn cost(&self) -> usize {
        self.action_history
            .iter()
            .map(|item| match item {
                TraverseAction::Forward => 1,
                TraverseAction::TurnLeft => 1001,
                TraverseAction::TurnRight => 1001,
            })
            .sum()
    }

    fn heuristic(&self, end_pos: Position) -> usize {
        let current_pos = self.pos_history.last().unwrap();
        end_pos.0.abs_diff(current_pos.0) + end_pos.1.abs_diff(current_pos.1)
    }

    fn total_estimated_cost(&self, end_pos: Position) -> usize {
        self.cost() + self.heuristic(end_pos)
    }

    fn get_current_direction(&self) -> Direction {
        let turns_count = self
            .action_history
            .iter()
            .map(|action| match action {
                TraverseAction::Forward => 0,
                TraverseAction::TurnLeft => -1,
                TraverseAction::TurnRight => 1,
            })
            .sum::<isize>()
            % (4);

        let current_dir = Direction::from(turns_count.rem_euclid(4));

        return current_dir;
    }

    fn get_current_pos(&self) -> Position {
        *self.pos_history.last().unwrap()
    }

    fn try_advance(&self) -> Vec<Path> {
        let current_dir = self.get_current_direction();
        let current_pos = self.get_current_pos();

        let mut next_paths = Vec::<Path>::new();

        for next_dir in [
            Direction::East,
            Direction::North,
            Direction::South,
            Direction::West,
        ] {
            let diff = ((next_dir as isize) - (current_dir as isize)).rem_euclid(4);

            // action would be going back
            if diff.abs() == 2 {
                continue;
            }

            let action = TraverseAction::from(diff);
            let next_pos_vec = next_dir.to_vec();
            let new_pos = (
                ((current_pos.0 as isize) + next_pos_vec.0) as usize,
                ((current_pos.1 as isize) + next_pos_vec.1) as usize,
            );

            let mut new_path = self.to_owned();
            new_path.action_history.push(action);
            new_path.cost += action.cost();
            new_path.pos_history.push(new_pos);

            next_paths.push(new_path);
        }

        return next_paths;
    }

    // pub fn get_dist_to(&self, other: &Self) -> usize{
    //     let self_pos = self.get_current_pos();
    //     let other_pos = other.get_current_pos();
    //     let advance_score = self_pos.0.abs_diff(other_pos.0) + self_pos.1.abs_diff(other_pos.1);

    //     let self_dir = self.get_current_direction();
    //     let other_dir = other.get_current_direction();
    //     let turn_score = 1000 * (self_dir as isize).abs_diff(other_dir as isize);

    //     return advance_score + turn_score;
    // }

    pub fn get_pos_history(&self) -> PositionHistory {
        self.pos_history.clone()
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        return (self.cost == other.cost)
            && (self.get_current_pos() == other.get_current_pos())
            && (self.get_current_direction() == other.get_current_direction());
    }
}

pub fn find_path(labyrinth: &Labyrinth) -> Option<Path> {
    let start_pos = get_start_pos(labyrinth);
    let end_pos = get_end_pos(labyrinth);

    let mut open_list = Vec::<Path>::new();
    let mut closed_list = Vec::<Position>::new();

    open_list.push(Path::new(start_pos));

    let mut paths = Vec::<Path>::new();

    while !open_list.is_empty() {
        open_list.sort_by(|left, right| {
            left.total_estimated_cost(end_pos)
                .cmp(&right.total_estimated_cost(end_pos))
        });
        open_list.reverse();

        let current_path = open_list.pop().unwrap();
        let current_pos = current_path.get_current_pos();

   //     draw_labyrinth_and_path_2(labyrinth, &current_path);

        if current_pos == end_pos {
            assert!(current_path.action_history.len() == current_path.pos_history.len() - 1);
            assert!(current_path.cost == current_path.cost());

            //return Some(current_path);
            paths.push(current_path);
            continue;
        }

        closed_list.push(current_pos);

        for neighbour in current_path.try_advance() {
            let next_pos = neighbour.get_current_pos();

            if labyrinth[next_pos.0][next_pos.1] == '#' {
                continue;
            }
            if closed_list.contains(&next_pos) {
                continue;
            }

            let visited_neighbour_idx = open_list
                .iter()
                .position(|path| path.get_current_pos() == current_pos);

            if visited_neighbour_idx.is_none() {
                open_list.push(neighbour);
                continue;
            }

            let visited_neighbour = open_list.get_mut(visited_neighbour_idx.unwrap()).unwrap();
            if visited_neighbour.cost > neighbour.cost {
                // newly discovered path to the same position is better
                *visited_neighbour = neighbour;
            }
        }
    }

    paths.sort_by(|left, right| left.cost.cmp(&right.cost));
    for path in paths.iter() {
        draw_labyrinth_and_path_2(labyrinth, path);
        println!()
    }
    return paths.first().cloned();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r"
            ###
            #.#
            ###        
            ";
        let labyrinth = parse_input(input);
        let path = find_path(&labyrinth).unwrap();
        assert!(path.cost == 0);

        let input = r"
            ####
            #..#
            ####        
            ";
        let labyrinth = parse_input(input);
        let path = find_path(&labyrinth).unwrap();
        assert!(path.cost == 1);

        let input = r"
            ###
            #.#
            #.#
            ###        
            ";
        let labyrinth = parse_input(input);
        let path = find_path(&labyrinth).unwrap();
        assert!(path.cost == 1001);

        let input = r"
            ####
            #..#
            #..#
            ####        
            ";
        let labyrinth = parse_input(input);
        let path = find_path(&labyrinth).unwrap();
        assert!(path.cost == 1002);

        let input = r"
            ###
            #.#
            ###
            #.#
            ###        
        ";
        let labyrinth = parse_input(input);
        let path = find_path(&labyrinth);
        assert!(path.is_none());

        let input = r"
            #####
            #...#
            #.#.#
            #...#
            #####        
        ";
        let labyrinth = parse_input(input);
        let path = find_path(&labyrinth).unwrap();
        assert!(path.cost == 1004);

        let input = r"
            #######
            #...#.#
            #...#.#
            #.#.#.#
            #.#.#.#
            #.#...#
            #.#...#
            #######        
        ";
        let labyrinth = parse_input(input);
        let path = find_path(&labyrinth).unwrap();
        draw_labyrinth_and_path_2(&labyrinth, &path);
        assert_eq!(path.cost, 5015);
    }

    #[test]
    fn or_does_it() {

        let input = r"
####################################################
#......................................#..........E#
#......................................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.............................#
#S...................#.............................#
####################################################   
        ";
        let labyrinth = parse_input(input);
        let path = find_path(&labyrinth).unwrap();
        
        draw_labyrinth_and_path_2(&labyrinth, &path);
        assert_eq!(path.cost, 5078);
    }
}
