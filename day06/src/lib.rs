#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn vector(&self) -> (i64, i64) {
        match *self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn_right(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    y: i64,
    x: i64,
}

impl Position {
    fn make_step(&self, dir: &Direction) -> Position {
        let vector = dir.vector();
        Position {
            y: self.y + vector.0,
            x: self.x + vector.1,
        }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }
}

impl PartialEq for Position{
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum MapObject {
    Nothing,
    Obstacle,
}

#[derive(Clone)]
pub struct LabMap {
    lab_map: Vec<Vec<MapObject>>,
}

impl LabMap {
    fn object_at(&self, pos: &Position) -> MapObject {
        self.lab_map[pos.y as usize][pos.x as usize]
    }

    fn width(&self) -> usize {
        self.lab_map[0].len()
    }

    fn height(&self) -> usize {
        self.lab_map.len()
    }

    fn is_invalid_pos(&self, pos: &Position) -> bool{
        pos.y < 0
        || pos.y >= self.height() as i64
        || pos.x < 0
        || pos.x >= self.width() as i64
    }

    pub fn clone_with_additional_obstacle(&self, pos: &Position) -> LabMap {
        let mut new_map = self.clone();
        new_map.lab_map[pos.y as usize][pos.x as usize] = MapObject::Obstacle;
        return new_map;
    }
}

pub fn parse_input(str: &str) -> (LabMap, Position) {
    let mut guard_pos = Position { y: 0, x: 0 };

    let lab_map: Vec<Vec<MapObject>> = str
        .trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|item| match item {
                    '#' => MapObject::Obstacle,
                    _ => MapObject::Nothing,
                })
                .collect()
        })
        .collect();

    for line in str.split("\n").enumerate() {
        let guard_pos_found = line.1.find("^");
        match guard_pos_found {
            Some(pos) => {
                guard_pos = Position {
                    y: line.0 as i64,
                    x: pos as i64,
                };
                break;
            }
            None => continue,
        }
    }

    (LabMap { lab_map }, guard_pos)
}

pub enum GuardStepResult {
    PatrolEnd,
    NextStep(Position, Direction),
}

pub fn simulate_one_guard_step(
    map: &LabMap,
    init_pos: &Position,
    init_dir: &Direction,
) -> GuardStepResult {

    let mut next_dir = init_dir.to_owned();
    let mut next_pos = init_pos.make_step(&next_dir);

    loop{
        let is_next_pos_out_of_bounds = map.is_invalid_pos(&next_pos);
        if is_next_pos_out_of_bounds {
            return GuardStepResult::PatrolEnd;
        }

        let is_facing_obstacle = map.object_at(&next_pos) == MapObject::Obstacle;
        if !is_facing_obstacle {
            break;
        }

        next_dir = next_dir.turn_right();
        next_pos = init_pos.make_step(&next_dir);
    }

    GuardStepResult::NextStep(next_pos, next_dir)
}


pub fn is_path_looping(map: &LabMap, guard_pos: &Position, guard_dir: &Direction) -> bool {

    let mut pos = guard_pos.to_owned();
    let mut dir = guard_dir.to_owned();

    let mut history: Vec<(Position, Direction)> = vec![(pos, dir)];

    loop {
        let result = simulate_one_guard_step(&map, &pos, &dir);
        match result {
            GuardStepResult::PatrolEnd => return false,
            GuardStepResult::NextStep(new_pos, new_dir) => {
                let already_visited = history.contains(&(new_pos, new_dir));
                if already_visited {
                    return true;
                }
                pos = new_pos;
                dir = new_dir;
                history.push((pos, dir));
            },
        }
    }
}