use std::env;
use crate::stuff::datatypes::Coord;

pub fn distance(a: Coord, b: Coord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}


pub fn min_distance(a: Coord, b: &[Coord]) -> i32 {
    b.iter()
        .map(|&coord| distance(a, coord))
        .fold(i32::MAX, i32::min)
}

pub enum AppMode {
    DEV,
    PROD
}

pub fn get_app_mode() -> AppMode {
    let env = env::var("APP_ENV").unwrap_or("dev".to_string()).to_ascii_lowercase();
    match env.as_str() {
        "dev" => AppMode::DEV,
        "prod" => AppMode::PROD,
        _ => panic!("UNKNOWN APP MODE!"),
    }
}