#[allow(unused_braces)]
use notan::{
    prelude::*,
    draw::{
        DrawConfig, 
        CreateFont, 
        Font,
        CreateDraw,
        Draw,
        DrawShapes,
        DrawTextSection
    }
};
#[allow(unused_braces)]
use crate::{
    walls::{
        Map,
    },
    player::{
        Player,
    }
};

mod walls;
mod player;
mod math;





const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;





struct Mouse {
    x: f32,
    y: f32,
}

impl Mouse {
    fn draw_cursor(&self, draw: &mut Draw) {
        draw.circle(5.0)
            .position(self.x, self.y)
            .color(Color::WHITE)
            .fill();
    }

    fn update_cursor(&mut self, app: &mut App) {
        (self.x, self.y) = app.mouse.position();
    }
}





#[derive(AppState)]
struct Game {
    font: Font,
    mouse: Mouse,
    delta_time: f32,
    map: Map,
    player: Player,
}

impl Game {
    fn init(app: &mut App, gfx: &mut Graphics) -> Self {
        let font = gfx
            .create_font(include_bytes!(".\\assets\\fonts\\OverpassMono-VariableFont_wght.ttf"))
            .unwrap();

        let (mouse_x, mouse_y) = app.mouse.position();

        app.window().set_cursor(CursorIcon::None);


        Game {
            font,
            mouse: Mouse {
                x: mouse_x,
                y: mouse_y
            },
            delta_time: 0.0,
            map: Map::init(),
            player: Player::init(),
        }
    }
}

fn update(app: &mut App, game: &mut Game) {
    if app.keyboard.was_pressed(KeyCode::Escape) {
        println!("Exiting game :(");
        app.exit();
    }
    game.delta_time = app.system_timer.delta().as_secs_f32();

    game.player.move_player(app, &mut game.map, game.delta_time);

    game.mouse.update_cursor(app);
}

fn draw(gfx: &mut Graphics, game: &mut Game) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::GRAY);

    // just so i don't get a warning for not using the font :3
    draw.text(&game.font, ":3")
        .position(SCREEN_WIDTH - 15., 15.)
        .color(Color::WHITE)
        .h_align_right();

    // draw the horizon
    let horizon = SCREEN_HEIGHT / 2.0;
    draw.line((0.0, horizon), (SCREEN_WIDTH, horizon))
        .color(Color{
            r: 0.329,
            g: 0.329,
            b: 0.329,
            a: 1.0,
        })
        .width(3.0);

    //game.player.draw_player(&mut draw);
    game.map.draw_map(&mut draw, &game.player);
    game.mouse.draw_cursor(&mut draw);

    gfx.render(&draw);
}


fn main() -> Result<(), String> {
    let window = WindowConfig::default()
        .set_size(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .set_decorations(true)
        .set_title("gmae");

    notan::init_with(Game::init)
        .add_config(window)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}
