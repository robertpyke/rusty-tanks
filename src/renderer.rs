use crate::components::LayeredSprite;
use crate::components::Position;
use crate::components::Sprite;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::join::Join;
use specs::ReadStorage;

// Type alias for the data needed by the renderer
pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, LayeredSprite>,
);

/// Private fn to render a sprite
fn render_sprite(
    canvas: &mut WindowCanvas,
    pos: &Position,
    sprite: &Sprite,
    textures: &[Texture],
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;

    // println!("Sprite: -> Result<(), String> {:?}", sprite);
    let current_frame = sprite.region;
    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(
        screen_position,
        current_frame.width(),
        current_frame.height(),
    );
    canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;

    Ok(())
}

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    data: SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    for (pos, sprite) in (&data.0, &data.1).join() {
        render_sprite(canvas, pos, sprite, textures)?;
    }

    for (pos, sprite_layer) in (&data.0, &data.2).join() {
        for sprite in &sprite_layer.sprites {
            render_sprite(canvas, pos, sprite, textures)?;
        }
    }

    canvas.present();

    Ok(())
}
