pub fn production_rate_per_hour(speed: u8) -> f64 {

    speed as f64 * 221.0 * match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=u8::MAX => 0.77,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let c = production_rate_per_hour(speed);
    (c / 60.0) as u32
}
