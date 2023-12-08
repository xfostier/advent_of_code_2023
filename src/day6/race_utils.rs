
// Rust

pub struct Race {
    pub race_time:u32,
    pub record_distance:u32
}

pub fn ways_to_win(race:&Race, ways: u32, button_time: u32) {
    // eliminate 0
    // eliminate max time
    if button_time == race.race_time {
        return;
    }
    if button_time*(race.race_time-button_time) > race.record_distance {
        return ways_to_win(race, ways+1, button_time+1)
    }
    return ways_to_win(race, ways, button_time+1)
}