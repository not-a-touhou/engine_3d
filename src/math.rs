use crate::walls::{
    Point,
    Line
};

pub fn distance(p1: &Point, p2: &Point) -> f32 {
    let length = (
        (p2.x - p1.x).powf(2.0) +
        (p2.y - p1.y).powf(2.0)
    ).sqrt();
    
    length
}

pub fn line_point(line: &Line, point: &Point) -> bool {
    let length = distance(&line.p1, &line.p2);

    let d1 = distance(&line.p1, &point);
    let d2 = distance(&line.p2, &point);

    let buffer: f32 = 0.5;

    if (d1 + d2 >= length - buffer) && (d1 + d2 <= length + buffer) {
        return true;
    } else {
        return false;
    }
}