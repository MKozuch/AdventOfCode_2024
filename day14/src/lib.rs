use std::collections::HashMap;
use regex::Regex;

pub struct Robot {
    pub p_x: u32,
    pub p_y: u32,
    pub v_x: i64,
    pub v_y: i64,
}

impl Robot {
    pub fn step(&mut self, dt: u32, map_width: u32, map_height: u32) {
        self.p_x = (self.p_x as i64 + self.v_x * dt as i64).rem_euclid(map_width as i64) as u32;
        self.p_y = (self.p_y as i64 + self.v_y * dt as i64).rem_euclid(map_height as i64) as u32;
    }
}

pub fn parse_input(input: &str) -> Vec<Robot> {
    input.trim().split('\n').map(parse_line).collect()
}

fn parse_line(input: &str) -> Robot {
    let rg = Regex::new(r"(?:^p=(\d+,\d+) v=(-?\d+,-?\d+)$)").unwrap();
    let captures = rg.captures(input).unwrap();

    Robot {
        p_x: captures[1]
            .split_once(',')
            .unwrap()
            .0
            .parse::<u32>()
            .unwrap(),
        p_y: captures[1]
            .split_once(',')
            .unwrap()
            .1
            .parse::<u32>()
            .unwrap(),
        v_x: captures[2]
            .split_once(',')
            .unwrap()
            .0
            .parse::<i64>()
            .unwrap(),
        v_y: captures[2]
            .split_once(',')
            .unwrap()
            .1
            .parse::<i64>()
            .unwrap(),
    }
}

pub fn calc_safety_factor(robots: &[Robot], field_width: u32, field_height: u32) -> u32 {
    let width_midpoint = (field_width - 1) / 2;
    let height_midpoint = (field_height - 1) / 2;

    let q1_count = robots
        .iter()
        .filter(|robot| robot.p_x < width_midpoint && robot.p_y < height_midpoint)
        .count() as u32;
    let q2_count = robots
        .iter()
        .filter(|robot| robot.p_x > width_midpoint && robot.p_y < height_midpoint)
        .count() as u32;
    let q3_count = robots
        .iter()
        .filter(|robot| robot.p_x < width_midpoint && robot.p_y > height_midpoint)
        .count() as u32;
    let q4_count = robots
        .iter()
        .filter(|robot| robot.p_x > width_midpoint && robot.p_y > height_midpoint)
        .count() as u32;

    let safety_factor = q1_count * q2_count * q3_count * q4_count;

    return safety_factor;
}

pub fn could_be_christmas_tree_1(robots: &[Robot], map_width: u32, _: u32) -> bool {
    let width_midpoint = (map_width - 1) / 2;
    let left_count = robots
        .iter()
        .filter(|robot| robot.p_x < width_midpoint)
        .count() as u32;
    let right_count = robots
        .iter()
        .filter(|robot| robot.p_x > width_midpoint)
        .count() as u32;

    return left_count == right_count;
}

pub fn could_be_christmas_tree_2(robots: &[Robot], _: u32, map_height: u32) -> bool {
    let mut map = HashMap::<u32, usize>::from_iter((0..map_height).map(|i| (i as u32, 0)));

    for robot in robots {
        *map.get_mut(&robot.p_y).unwrap() += 1;
    }

    return map.values().all(|item| *item == 2 || *item == 0);
}

pub fn could_be_christmas_tree_3(robots: &[Robot]) -> bool {
    let coords_iter = robots.iter().map(|robot| (robot.p_x, robot.p_y));
    let mut coords_vec = Vec::from_iter(coords_iter);
    coords_vec.sort();
    coords_vec.dedup();

    return coords_vec.len() == robots.len();
}

pub fn draw_robots(robots: &[Robot], field_width: u32, field_height: u32) {
    let mut field = Vec::<Vec<u8>>::new();
    let line: Vec<u8> = (0..field_width).map(|_| 0).collect();

    for _ in 0..field_height {
        field.push(line.clone());
    }

    for robot in robots {
        field[robot.p_y as usize][robot.p_x as usize] += 1;
    }

    for line in field {
        let str = String::from_iter(line.iter().map(|item| match item {
            0 => '.',
            i => char::from_digit(*i as u32, 10).unwrap(),
        }));
        println!("{}", str);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut robot = Robot {
            p_x: 2,
            p_y: 4,
            v_x: 2,
            v_y: -3,
        };
        let map_width = 11;
        let map_height = 7;

        robot.step(1, map_width, map_height);
        assert_eq!(robot.p_x, 4);
        assert_eq!(robot.p_y, 1);

        robot.step(1, map_width, map_height);
        assert_eq!(robot.p_x, 6);
        assert_eq!(robot.p_y, 5);

        robot.step(3, map_width, map_height);
        assert_eq!(robot.p_x, 1);
        assert_eq!(robot.p_y, 3);
    }
}
