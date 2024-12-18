use std::fs;

use day16::*;

fn main() {
    let input_path = "input/input.txt";
    let input_path = "input/test_input.txt";

    let input = fs::read_to_string(input_path).unwrap();

    let labyrinth = parse_input(&input);

    let start_pos = get_start_pos(&labyrinth);
    let end_pos = get_end_pos(&labyrinth);

    let mut active_paths = Vec::<(ActionHistory, PositionHistory)>::new();
    active_paths.push((vec![], vec![start_pos]));
    
    let mut finished_paths: Vec<PositionHistory> = vec![];

    'loop: loop {
        if active_paths.is_empty() {
            break 'loop;
        }

        for i in 0..active_paths.len() {
            let (mut action_history, mut position_history) = active_paths.get(i).unwrap();
            let available_actions = get_next_actions(&labyrinth, &action_history, &position_history);

            if(available_actions.is_empty()){
                active_paths.remove(i);
                continue 'loop;
            }
            
            // action_history.append(&mut available_actions.get(0).unwrap().0);
            // position_history.append(&mut available_actions.get(0).unwrap().1);

            // for (new_actions, new_pos) in available_actions.iter().skip(1) {

            for (new_actions, new_pos) in available_actions.iter() {
                let mut new_action_history = action_history.clone();
                new_action_history.append(&mut new_actions.clone());

                let mut new_position_histoy = position_history.clone();
                new_position_histoy.push(new_pos);

                if new_pos == end_pos{
                    finished_paths.push(new_action_history);
                }
                else{
                    active_paths.push((new_action_history, new_position_histoy));
                    active_paths.remove(i);
                    continue 'loop;
                }
            }

            // todo: return turn+forward and new position from get_next_actions

            // if new pos is end then add history to finished_paths and remove current active path
            // if no possible actions remove current active path

            // append new actions at the end of this path and push at the end of active_paths, remove this path
                
            }

        }
    }



    //println!("Result: {result}");
}

