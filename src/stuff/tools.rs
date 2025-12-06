use crate::stuff::datatypes::Coord;

pub fn distance(a: Coord, b: Coord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}


pub fn min_distance(a: Coord, b: &[Coord]) -> i32 {
    b.iter()
        .map(|&coord| distance(a, coord))
        .fold(i32::MAX, i32::min)
}
