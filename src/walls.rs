use notan::{
    draw::{
        Draw, 
        DrawShapes
    }, 
    app::Color
};

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};


#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point {
            x,
            y
        }
    }

    pub fn translate(&mut self, x_dist: f32, y_dist: f32) {
        self.x += x_dist;
        self.y += y_dist;
    }

    pub fn rotate(&mut self, angle: f32) {
        self.x = self.x * angle.cos() - self.y * angle.sin();
        self.y = self.x * angle.sin() + self.y * angle.cos();
    }
}

#[derive(Clone)]
pub struct Line {
    pub p1: Point,
    pub p2: Point
}

impl Line {
    fn new_line(p1: Point, p2: Point) -> Self {
        Line {
            p1,
            p2
        }
    }

    fn draw_line(line: &Line, draw: &mut Draw) {
        draw.line(
            (line.p1.x + SCREEN_WIDTH / 2., line.p1.y + SCREEN_HEIGHT / 2.), 
            (line.p2.x + SCREEN_WIDTH / 2., line.p2.y + SCREEN_HEIGHT / 2.)
        )
            .color(Color::WHITE)
            .width(3.0);
    }


    fn translate(&mut self, x_dist: f32, y_dist: f32) {
        self.p1.translate(x_dist, y_dist);
        self.p2.translate(x_dist, y_dist);
    }

    fn rotate(&mut self, angle: f32) {
        self.p1.rotate(angle);
        self.p2.rotate(angle);
    }
}

#[derive(Clone)]
pub struct Map {
    pub vec: Vec<Line>
}

impl Map {
    pub fn init() -> Self {
        Map {
            vec: vec![
                // triangle
                Line::new_line(Point::new(15.0, 15.0), Point::new(10.0, 480.0)),
                Line::new_line(Point::new(10.0, 480.0), Point::new(740.0, 890.0)),
                Line::new_line(Point::new(740.0, 890.0), Point::new(15.0, 15.0)),

                //square
                Line::new_line(Point::new(1000.0, 1100.0), Point::new(1000.0, 1500.0)),
                Line::new_line(Point::new(1000.0, 1500.0), Point::new(600.0, 1500.0)),
                Line::new_line(Point::new(600.0, 1500.0),  Point::new(600.0, 1100.0)),
                Line::new_line(Point::new(600.0, 1100.0),  Point::new(1000.0, 1100.0)),
            ]
        }
    }

    pub fn draw_map(&self, draw: &mut Draw) {
        for line in &self.vec {
            Line::draw_line(line, draw);
        }
    }

    pub fn translate(&mut self, x_dist: f32, y_dist:f32) {
        for line in &mut self.vec {
            line.translate(x_dist, y_dist);
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        for line in &mut self.vec {
            line.rotate(-angle)
        }
    }
}

