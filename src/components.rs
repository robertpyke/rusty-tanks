use sdl2::rect::{Point, Rect};
use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

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
    pub sprites: Vec<Sprite>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct KeyboardControlled {
    pub speed: i32,
}

/// The current speed and direction of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}
