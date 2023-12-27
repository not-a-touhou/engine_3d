use notan::{
    draw::{
        Draw, 
        DrawShapes,
    }, 
    app::Color
};

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, player::Player};
const WALL_HEIGHT: f32 = 100.0;


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

    pub fn to_draw(&self) -> Self {
        let depth = self.y;
        let depth_scale = 1.0 / depth * SCREEN_HEIGHT;

        let mut transformed = Point::new(self.x, WALL_HEIGHT/2.0);

        transformed.x *= depth_scale;
        transformed.y *= depth_scale;

        transformed.x += SCREEN_WIDTH / 2.0;
        transformed.y += 0.0;

        return transformed
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
        let p1 = line.p1.to_draw();
        let p2 = line.p2.to_draw();

        let vcenter = SCREEN_HEIGHT / 2.0;

        // bottom edge
        draw.line(
            (p1.x, vcenter + p1.y),
            (p2.x, vcenter + p2.y)
        ).color(Color::WHITE).width(3.0);

        // top edge
        draw.line(
            (p1.x, (vcenter - p1.y)),
            (p2.x, (vcenter - p2.y))
        ).color(Color::WHITE).width(3.0);
        
        // vertical lines
        draw.line(
            (p1.x, (vcenter - p1.y)),
            (p1.x, (vcenter + p1.y))
        ).color(Color::WHITE).width(3.0);
        draw.line(
            (p2.x, (vcenter - p2.y)),
            (p2.x, (vcenter + p2.y))
        ).color(Color::WHITE).width(3.0);
    }


    fn translate(&mut self, x_dist: f32, y_dist: f32) {
        self.p1.translate(x_dist, y_dist);
        self.p2.translate(x_dist, y_dist);
    }

    fn rotate(&mut self, angle: f32) {
        self.p1.rotate(angle);
        self.p2.rotate(angle);
    }

    #[allow(dead_code)]
    fn clip_line(&self, player: &Player) -> Self {
        let (front, back) = if self.p1.y < player.clip_depth {
            (&self.p1, &self.p2)
        } else {
            (&self.p2, &self.p1)
        };

        // vertical size of the line
        let size = front.y - back.y;
        // divide point in front by size to get a percentage
        let percentage = front.y / size;
        // set the x as itself multiplied by the percentage of the line we can see
        let clip_x = front.x + (back.x - front.x) * percentage;

        return Line::new_line(front.clone(), Point { x: clip_x, y: player.clip_depth })
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

    pub fn draw_map(&mut self, draw: &mut Draw, player: &Player) {
        for line in &mut self.clip_walls(player).clone() {
            Line::draw_line(&line, draw);
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

    fn clip_walls(&mut self, player: &Player) -> Vec<Line> {
        let mut vec = Vec::new();

        for line in &self.vec {
            // is the line entirely in front of us?
            if line.p1.y <= player.clip_depth && line.p2.y <= player.clip_depth {
                vec.push(line.clone());
            // is the line entirely behind us?
            } else if line.p1.y > player.clip_depth && line.p2.y > player.clip_depth {
                continue;
            // if not, then it must be half in front, half behind.
            // therefore we must clip it
            } else {
                //vec.push(line.clip_line(&player));
                continue;
                
            }
        }

        vec
    }

}

