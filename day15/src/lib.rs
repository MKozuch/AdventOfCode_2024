#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(chr: char) -> Direction {
        match chr {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!(),
        }
    }

    fn to_vector(&self) -> (i64, i64) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

type DirectionList = Vec<Direction>;

#[derive(PartialEq, Clone, Copy)]
pub enum WarehouseItem {
    Nothing,
    Box,
    Wall,
    Robot,
}

impl WarehouseItem {
    fn from_char(chr: char) -> WarehouseItem {
        match chr {
            '.' => WarehouseItem::Nothing,
            'O' => WarehouseItem::Box,
            '#' => WarehouseItem::Wall,
            '@' => WarehouseItem::Nothing,
            _ => panic!(),
        }
    }

    fn to_char(&self) -> char {
        match self {
            WarehouseItem::Nothing => '.',
            WarehouseItem::Box => 'O',
            WarehouseItem::Wall => '#',
            WarehouseItem::Robot => '@',
        }
    }
}

pub type WarehouseMap = Vec<Vec<WarehouseItem>>;

impl std::fmt::Display for WarehouseItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy)]
pub struct WarehousePos {
    pub pos_y: usize,
    pub pos_x: usize,
}

impl WarehousePos {
    pub fn next_pos(&self, dir: Direction) -> WarehousePos {
        let (d_y, d_x) = dir.to_vector();
        WarehousePos {
            pos_y: ((self.pos_y as i64) + d_y) as usize,
            pos_x: ((self.pos_x as i64) + d_x) as usize,
        }
    }
}

pub fn draw_warehouse_map(warehouse_map: &WarehouseMap, robot: Option<WarehousePos>) {
    //let output = Vec::<String>::new();

    for (line_idx, line) in warehouse_map.iter().enumerate() {
        let mut line = String::from_iter(line.iter().map(WarehouseItem::to_char));

        if let Some(robot) = robot {
            if robot.pos_y == line_idx {
                let pos = robot.pos_x as usize;
                line.replace_range(pos..pos + 1, "@");
            }
        }

        println!("{}", line);
    }
}

pub fn simulate_robot_move(
    warehouse_map: &mut WarehouseMap,
    robot: &mut WarehousePos,
    move_dir: Direction,
) {
    let next_robot_step = robot.next_pos(move_dir);
    let item_at_new_pos = warehouse_map[next_robot_step.pos_y][next_robot_step.pos_x];

    match item_at_new_pos {
        WarehouseItem::Nothing => *robot = next_robot_step,
        WarehouseItem::Wall => {}
        WarehouseItem::Robot => panic!(),
        WarehouseItem::Box => {
            let mut next_item_pos = next_robot_step.next_pos(move_dir);
            let mut next_item = warehouse_map[next_item_pos.pos_y][next_item_pos.pos_x];

            loop {
                if next_item == WarehouseItem::Nothing {
                    warehouse_map[next_robot_step.pos_y][next_robot_step.pos_x] = WarehouseItem::Nothing;
                    warehouse_map[next_item_pos.pos_y][next_item_pos.pos_x] = WarehouseItem::Box;
                    *robot = next_robot_step;
                    return;
                } else if next_item == WarehouseItem::Box {
                    next_item_pos = next_item_pos.next_pos(move_dir);
                    next_item = warehouse_map[next_item_pos.pos_y][next_item_pos.pos_x];
                } else if next_item == WarehouseItem::Wall {
                    break;
                }
            }
        }
    }
}

pub fn parse_directions(input: &str) -> DirectionList {
    input
        .trim()
        .split("\n")
        .map(|line| line.chars())
        .flatten()
        .map(|chr| Direction::from_char(chr))
        .collect::<Vec<_>>()
}

pub fn parse_map(input: &str) -> WarehouseMap {
    let lines_iter = input.trim().split("\n");

    let parse_line = |line: &str| {
        line.chars()
            .map(WarehouseItem::from_char)
            .collect::<Vec<_>>()
    };

    lines_iter.map(parse_line).collect()
}

pub fn find_robot(input: &str) -> WarehousePos {
    for (line_num, line) in input.split("\n").enumerate() {
        if let Some(index) = line.chars().position(|chr| chr == '@') {
            return WarehousePos {
                pos_y: line_num,
                pos_x: index,
            };
        }
    }

    panic!();
}

pub fn parse_input(input: &str) -> (WarehouseMap, DirectionList, WarehousePos) {
    let (warehouse_input, directions_input) = input.split_once("\n\n").unwrap();

    let direction_list = parse_directions(directions_input);
    let warehouse_map = parse_map(warehouse_input);
    let robot = find_robot(warehouse_input);

    return (warehouse_map, direction_list, robot);
}

pub fn calc_gps_coords(warehouse_map: &WarehouseMap) -> u64 {
    let mut sum = 0_u64;

    for (line_idx, line) in warehouse_map.iter().enumerate() {
        for (item_idx, item) in line.iter().enumerate() {
            if *item == WarehouseItem::Box {
                sum += (100 * line_idx as u64) + (item_idx as u64);
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_gps_coords_test() {
        let input = r"
#######
#...O..
#......
";
        let warehouse_map = parse_map(input);
        let gps_score = calc_gps_coords(&warehouse_map);
        assert_eq!(gps_score, 104);

        let input = r"
##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
";
        let warehouse_map = parse_map(input);
        let gps_score = calc_gps_coords(&warehouse_map);
        assert_eq!(gps_score, 10092);
    }
}
