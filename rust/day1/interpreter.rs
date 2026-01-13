use crate::dial;

pub fn interpret_password(
    dial: &(i32, i32, i32),
    stops: &Vec<(&(char, i32), i32)>,
    verbose: bool
) -> i32 {
    let (dial_min, dial_max, dial_start) = dial;

    let mut num_stops_at_zero = 0;

    if verbose {
        println!("Dial: {}...{}", dial_min, dial_max);
        println!("Start: {}", dial_start);
    }

    for turn in stops {
        let (_cause, stop) = turn;
        
        if *stop == 0 {
            num_stops_at_zero += 1;
        }

        if verbose {
            log_turn(turn);
        }
    }

    return num_stops_at_zero;
}

fn log_turn(turn: &(&(char, i32), i32)) {
    let (cause, stop) = turn;
    let (turn_dir, turn_amnt) = cause;

    if *turn_dir == dial::INSTR_LEFT {
        println!("<- {amnt}: {stop}", amnt=turn_amnt, stop=stop);
    } else {
        println!("-> {amnt}: {stop}", amnt=turn_amnt, stop=stop);
    }
}
