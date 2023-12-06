
// Rust

pub struct Race {
    pub race_time:u8,
    pub record_distance:u8
}

pub fn ways_to_win(race:Race, ways: &u8, button_time: u8) {
    // eliminate 0
    // eliminate max time
    if button_time == race.race_time {
        return;
    }
    if button_time*(race.race_time-button_time) > race.record_distance {
        *ways += 1;
    }
    return ways_to_win(race, ways, button_time+1)
}