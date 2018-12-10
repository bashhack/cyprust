use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, draw_context: &Context, graphics_buffer: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    let draw_state = [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE];

    rectangle(color, draw_state, draw_context.transform, graphics_buffer);
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    draw_context: &Context,
    graphics_buffer: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        draw_context.transform,
        graphics_buffer,
    );
}
