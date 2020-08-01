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



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    Clockwise,
    CounterClockwise
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct KeyboardControlled {
    pub speed: i32,
    pub rotation_speed: i32
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Angle {
    pub angle: i32
}

/// The current speed and Direction of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}

/// The current speed and Rotation of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct AngularVelocity {
    pub speed: i32,
    pub rotation: Rotation,
}