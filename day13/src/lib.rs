use regex::Regex;

#[derive(Clone, Copy)]
pub struct Machine{
    pub button_a: (u64, u64),
    pub button_b: (u64, u64),
    pub prize: (u64, u64)
}

fn parse_machine(input: &str) -> Machine {
    let re = Regex::new(r"^(?:Button A: X\+(\d+), Y\+(\d+))\n(?:Button B: X\+(\d+), Y\+(\d+))\n(?:Prize: X=(\d+), Y=(\d+))$").unwrap();

    let caps = re.captures(input).unwrap();

    Machine{
        button_a: (
            caps[1].parse::<u64>().unwrap(),
            caps[2].parse::<u64>().unwrap(),
        ),
        button_b: (
            caps[3].parse::<u64>().unwrap(),
            caps[4].parse::<u64>().unwrap(),
        ),
        prize: (
            caps[5].parse::<u64>().unwrap(),
            caps[6].parse::<u64>().unwrap(),
        )
    }
}

pub fn parse_input(input: &str) -> Vec<Machine> {
    input
        .trim()
        .split("\n\n")
        .map(parse_machine)
        .collect()
}

pub fn calc_winning_cost(machine: &Machine) -> Option<u64> {
    let mut target_x = machine.prize.0 as f64;
    let mut target_y = machine.prize.1 as f64;
    let mut a_x = machine.button_a.0 as f64;
    let mut a_y = machine.button_a.1 as f64;
    let mut b_x = machine.button_b.0 as f64;
    let mut b_y = machine.button_b.1 as f64;

    let mul_1 = a_x;
    let mul_2 = a_y;

    target_x *= mul_2;
    a_x *= mul_2;
    b_x *= mul_2;

    target_y *= mul_1;
    a_y *= mul_1;
    b_y *= mul_1;

    assert_eq!(a_x, a_y);

    let target = (target_x - target_y).abs();
    let b = target / (b_x-b_y).abs();
    let a = (b_x * b - target_x).abs() / a_x;

    if a.fract() != 0.0 || b.fract() != 0.0{
        println!("No solution");
        return None;
    }

    let cost_a = 3;
    let cost_b = 1;
    let a = a.round() as u64;
    let b = b.round() as u64;

    let cost = cost_a * a + cost_b * b;

    println!("a: {}, b: {}, cost: {}", a, b, cost);

    return Some(cost);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1,1);
    }
}