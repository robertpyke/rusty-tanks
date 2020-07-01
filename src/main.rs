mod renderer;
mod components;

use crate::components::Direction;
use specs::world::Builder;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};

use specs::prelude::{Dispatcher, DispatcherBuilder, World, SystemData};

use crate::components::{Position, Sprite, LayeredSprite};

pub enum MovementCommand {
    Stop,
    Move(Direction),
}

fn initialize_tank(world: &mut World, tank_base_sprite: usize, tank_turret_sprite: usize) {
    let a = Sprite {spritesheet: tank_base_sprite, region: Rect::new(0, 0, 32, 32)};
    let b = Sprite {spritesheet: tank_turret_sprite, region: Rect::new(0, 0, 32, 32)};
    let sprites = vec![a,b];
    world.create_entity()
        .with(Position(Point::new(0, 0)))
        .with(LayeredSprite {sprites: sprites})
        .build();
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rusty-tanks", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    let mut dispatcher = DispatcherBuilder::new()
        .build();
    

    let mut world = World::new();
    dispatcher.setup(&mut world.res);
    renderer::SystemData::setup(&mut world.res);

    // Initialize resource
    let movement_command: Option<MovementCommand> = None;
    world.add_resource(movement_command);

    let textures = [
        texture_creator.load_texture("resources/assets/tank/bullet.png")?,
        texture_creator.load_texture("resources/assets/tank/tankBase.png")?,
        texture_creator.load_texture("resources/assets/tank/tankTurret.png")?,
    ];

    initialize_tank(&mut world, 1, 2);

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // None - no change, Some(MovementCommand) - perform movement
        let mut movement_command = None;

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },

                // go
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                },

                // stop
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Stop);
                },
                _ => {}
            }
        }

        *world.write_resource() = movement_command;

        // Update
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        // Render
        i = (i + 1) % 255;
        renderer::render(&mut canvas, Color::RGB(i, 64, 255 - i), &textures, world.system_data())?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}