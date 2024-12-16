use std::{
    fs,
    io::{stdin, Read},
};

use day14::*;

fn main() {
    let test = true;

    let input_path = match test {
        true => "input/test_input.txt",
        false => "input/input.txt",
    };

    let (map_width, map_height) = match test {
        true => (11, 7),
        false => (101, 103),
    };

    let input = fs::read_to_string(input_path).unwrap();

    let mut robots = parse_input(&input);

    for robot in robots.iter_mut() {
        robot.step(100, map_width, map_width);
    }

    let result = calc_safety_factor(&robots, map_width, map_width);

    println!("Result: {result}");

    let mut robots = parse_input(&input);
    let mut i = 0;
    loop {
        robots
            .iter_mut()
            .for_each(|robot| robot.step(1, map_width, map_height));
        i += 1;

        if could_be_christmas_tree_3(&robots) {
            println!("Iteration {}", i);
            draw_robots(&robots, map_width, map_height);
            println!("Is this your picture?");
            let _ = stdin().read(&mut [0u8]).unwrap();
        } else {
            if i % 10000 == 0 {
                println!("Iteration {}", i);
            }
        }
    }
}
