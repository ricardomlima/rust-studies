// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PER_SPEED_RATIO: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed: u32 = speed.into();
    let brute_production: f64 = (CARS_PER_SPEED_RATIO * speed).into();
    let mut production_rate: f64 = 0.0;

    if speed >= 1 && speed <= 4 {
        production_rate = brute_production;
    }

    if speed >= 5 && speed <= 8 {
        production_rate = brute_production * 0.9;
    }

    if speed >= 9 && speed <= 10 {
        production_rate = brute_production * 0.77;
    }

    production_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let production_rate_per_hour: f64 = production_rate_per_hour(speed);
    let production_rate_per_minute: u32 = (production_rate_per_hour / 60.0) as u32;
    production_rate_per_minute
}