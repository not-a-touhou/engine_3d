use notan::{draw::{Draw, DrawShapes}, app::{Color, App}, input::keyboard::KeyCode};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, walls::{Map, Line, Point}, math::{distance, line_point}};

pub struct Player {
    x: f32,
    y: f32,
    proj_x: f32,
    proj_y: f32,
    can_move: bool,
    radius: f32,
    speed: f32,
    velocity: (f32, f32),
}

impl Player {
    pub fn init() -> Self {
        Player {
            x: 0.0,
            y: 0.0,
            proj_x: 0.,
            proj_y: 0.,
            can_move: true,
            radius: 15.,
            speed: 300.,
            velocity: (0., 0.)
        }
    }


    pub fn draw_player(&self, draw: &mut Draw) {
        draw.circle(self.radius)
            .position(self.x + SCREEN_WIDTH / 2.0, self.y + SCREEN_HEIGHT / 2.0)
            .color(Color::TEAL);

        draw.line((self.x + SCREEN_WIDTH / 2.0, self.y + SCREEN_HEIGHT / 2.0), (self.x + SCREEN_WIDTH / 2.0, self.y - 35.0 + SCREEN_HEIGHT / 2.0))
            .color(Color::TEAL)
            .width(5.0);
    }

    pub fn move_player(&mut self, app: &mut App, map: &mut Map, dt: f32) {
        self.velocity = (0., 0.);

        if app.keyboard.is_down(KeyCode::A) {
            (self.velocity.0, self.velocity.1) = (self.speed, self.velocity.1);
        }
        if app.keyboard.is_down(KeyCode::D) {
            (self.velocity.0, self.velocity.1) = (-self.speed, self.velocity.1);
        }

        if app.keyboard.is_down(KeyCode::W) {
            (self.velocity.0, self.velocity.1) = (self.velocity.0, self.speed);
        }
        if app.keyboard.is_down(KeyCode::S) {
            (self.velocity.0, self.velocity.1) = (self.velocity.0, -self.speed);
        }

        if app.keyboard.is_down(KeyCode::Left) {
            map.rotate(-0.005);
        }
        if app.keyboard.is_down(KeyCode::Right) {
            map.rotate(0.005);
        }

        self.velocity.0 *= dt;
        self.velocity.1 *= dt;

        self.proj_x = self.x - self.velocity.0;
        self.proj_y = self.y - self.velocity.1;

        self.can_move = true;

        for line in &map.vec {
            let colliding = self.wall_collision(line.clone());
            if colliding.0 {
                self.can_move = false;
                self.velocity = (colliding.1 / 16., colliding.2 / 16.);
            }
        }


        map.translate(self.velocity.0, self.velocity.1);

        
    }

    pub fn wall_collision(&self, line: Line) -> (bool, f32, f32) {
        // get length of line we are colliding with
        let length_ab = distance(&line.p1, &line.p2);

        // find the dot product of the two vectors
        // (a-b, a-circle)
        let dot = ( 
            ((self.proj_x - line.p1.x) * (line.p2.x - line.p1.x)) + 
            ((self.proj_y - line.p1.y) * (line.p2.y - line.p1.y)) 
        ) / (length_ab.powf(2.0));

        // use the dot product to find the closest point somehow
        let closest: Point = Point {
            x: line.p1.x + (dot * (line.p2.x - line.p1.x)),
            y: line.p1.y + (dot * (line.p2.y - line.p1.y))
        };

        // now we need to check if the point we are checking is actually on the line
        if !line_point(&line, &closest) {
            return (false, 0., 0.);
        }

        let dist: f32 = distance(
            &closest,
            &Point {
                x: self.proj_x,
                y: self.proj_y
            }
        );


        if dist < self.radius {
            return (true, closest.x - self.proj_x, closest.y - self.proj_y);
        } else {
            return (false, 0., 0.);
        }
    }
}