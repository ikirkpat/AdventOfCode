pub const INSTR_LEFT: char = 'l';
pub const INSTR_RIGHT: char = 'r';

pub fn dial(start: i32, min: i32, max: i32, instructions: &Vec<(char, i32)>) -> Vec<(&(char, i32), i32)> {
    let mut stops: Vec<(&(char, i32), i32)> = Vec::new();
    let mut last_stop: i32 = start;

    for instr in instructions {
        let stop_result: Result<i32, String> = turn(last_stop, min, max, instr);

        match stop_result {
            Ok(stop) => {
                let stop_w_cause = (instr, stop);
                stops.push(stop_w_cause);
                last_stop = stop;
            }
            Err(error_msg) => {
                eprintln!("Could not carry out instruction \"{:#?}\": {}", instr, error_msg);
            }
        }

    }

    return stops;
}

fn turn(current: i32, min: i32, max: i32, instruction: &(char, i32)) -> Result<i32, String> {
    let (direction, value) = instruction;

    if *direction == INSTR_LEFT {
        if current < *value {
            let left: i32 = value - current;
            return turn(max + 1, min, max, &(*direction, left));
        } else {
            let new_current: i32 = current - value;
            return Ok(new_current);
        }
    } else if *direction == INSTR_RIGHT {
        let new_current: i32 = current + value;
        let num_options = max - min + 1; // Needs to be inclusive of min and max so add 1
        return Ok(new_current % num_options);
    } else {
        return Err(format!(
            "Direction must be one of ['{left}', '{right}'], not '{actual}'.",
            left = INSTR_LEFT,
            right = INSTR_RIGHT,
            actual = direction));
    }
}
