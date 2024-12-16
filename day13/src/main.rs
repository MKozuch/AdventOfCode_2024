use std::fs;

use day13::*;

fn main() {
    let input_path = "input/input.txt";
  //  let input_path = "input/test_input.txt";

    let input = fs::read_to_string(input_path).unwrap();
    
    let machines = parse_input(&input);


    let mut total_cost = 0_u64;
    for machine in &machines{
        match calc_winning_cost(&machine){
            Some(cost) => total_cost += cost,
            None => continue,
        }
    }
    println!("Result: {total_cost}");


    let corrected_machines = 
        machines.iter().map(|machine| {
            let mut m = machine.to_owned();
            m.prize = (machine.prize.0 + 10000000000000, machine.prize.1 + 10000000000000);
            return m;
    }).collect::<Vec<_>>();

    let mut total_cost = 0_u64;
    for machine in &corrected_machines{
        match calc_winning_cost(&machine){
            Some(cost) => total_cost += cost,
            None => continue,
        }
    }

   println!("Result2: {total_cost}");
}
