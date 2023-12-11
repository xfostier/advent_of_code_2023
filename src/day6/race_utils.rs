
// Rust

use std::num::Wrapping;

pub struct Race {
    pub race_time:u64,
    pub record_distance:u64
}

pub fn ways_to_win_simple_recursive(race:&Race, ways: u64, button_press: u64) -> u64 {
    // eliminate 0
    // eliminate max time
    if button_press == race.race_time {
        return ways;
    }
    if is_race_won(race, button_press) {
        println!("PROGRESS..{}",button_press);
        return ways_to_win_simple_recursive(race, ways+1, button_press+1)
    }
    println!("PROGRESS..{}",button_press);
    return ways_to_win_simple_recursive(race, ways, button_press+1);
}

// TODO: Rework this garbage
fn search_bounds(race: &Race, start: u64, end: u64) -> (Option<u64>, Option<u64>){
    println!("PROGRESS..{}..{}",start, end);
    if is_race_won(race, start) {
        return (Some(start), None);
    } else if is_race_won(race, end) {
        return (None, Some(end));
    } else {
        return search_bounds(race, start+1, end-1)
    }
}

fn is_race_won(race: &Race, button_press: u64) -> bool {
    let wrapped_button = Wrapping(button_press);
    let wrapped_time = Wrapping(race.race_time);
    let wrapped_record = Wrapping(race.record_distance);
    let result = wrapped_button*(wrapped_time-wrapped_button);
    if result > wrapped_record {
        return true
    }
    return false;
}