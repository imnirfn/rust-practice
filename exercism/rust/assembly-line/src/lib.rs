// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
//

const CAR_PRODUCED_PER_HOUR: u8 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let prod_rate: f64;
    prod_rate = speed as f64 * CAR_PRODUCED_PER_HOUR as f64;
    match speed {
        1..=4 => {
            let prod_rate = (prod_rate * 100.0).round() / 100.0;
            prod_rate
        },
        5..=8 => {
            let prod_rate = (prod_rate * 90.0).round() / 100.0;
            prod_rate
        },
        9..=10 => {
            let prod_rate = (prod_rate * 77.0).round() / 100.0;
            prod_rate
        },
        _ => 0.0
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let prod_rate: f64;
    prod_rate = (speed as f64 * (CAR_PRODUCED_PER_HOUR as f64 /60.0)) as f64;
    match speed {
        1..=4 => {
            let prod_rate = (prod_rate as f64 * 100.0).round() / 100.0;
            prod_rate as u32
        },
        5..=8 => {
            let prod_rate = (prod_rate as f64 * 90.0).round() / 100.0;
            prod_rate as u32
        },
        9..=10 => {
            let prod_rate = (prod_rate as f64 * 77.0).round() / 100.0;
            prod_rate as u32
        },
        _ => 0
    }
}
