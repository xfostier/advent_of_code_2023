
// Rust

pub struct Race {
    pub race_time:u64,
    pub record_distance:u64
}

pub fn ways_to_win_simple_recursive(race:&Race, ways: u64, button_time: u64) -> u64 {
    // eliminate 0
    // eliminate max time
    if button_time == race.race_time {
        return ways;
    }
    if button_time*(race.race_time-button_time) > race.record_distance {
        println!("PROGRESS..{}",button_time);
        return ways_to_win_simple_recursive(race, ways+1, button_time+1)
    }
    println!("PROGRESS..{}",button_time);
    return ways_to_win_simple_recursive(race, ways, button_time+1)
}

pub fn ways_to_win_divide_and_conquer(race:&Race, ways: u64, button_time: u64) -> u64 { // TODO
    // eliminate 0
    // eliminate max time
    if button_time == race.race_time {
        return ways;
    }
    if button_time*(race.race_time-button_time) > race.record_distance {
        println!("PROGRESS..{}",ways);
        return ways_to_win_simple_recursive(race, ways+1, button_time+1)
    }
    println!("PROGRESS..{}",ways);
    return ways_to_win_simple_recursive(race, ways, button_time+1)
}