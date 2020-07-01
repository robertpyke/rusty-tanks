use specs::VecStorage;
use specs::Component;
use specs_derive::Component;
use sdl2::rect::{Point, Rect};

/// The current position of a given entity
/// https://docs.rs/specs/0.16.1/specs/storage/index.html
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);


#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    /// The specific spritesheet to render from
    pub spritesheet: usize,
    /// The current region of the spritesheet to be rendered
    pub region: Rect,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct LayeredSprite {
    pub sprites: Vec<Sprite>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}