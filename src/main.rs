mod components;
mod keyboard;
mod physics;
mod renderer;
mod resources;
mod bulletspawner_system;
use crate::components::BulletSpawner;
use specs::prelude::*;

use crate::components::Angle;
use crate::components::AngularVelocity;
use crate::components::Rotation;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use specs::world::Builder;
use std::time::Duration;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use specs::prelude::{DispatcherBuilder, SystemData, World};

use crate::components::{KeyboardControlled, Position, Sprite, Velocity};

pub enum RotationCommand {
    Stop,
    Move(Rotation),
}

pub enum MovementCommand {
    Stop,
    Move(Angle),
}

pub enum FireCommand {
    Stop,
    Fire,
}

fn initialize_tank(world: &mut World, tank_base_sprite: usize, tank_turret_sprite: usize) {
    // Init the base
    world
        .create_entity()
        .with(Position(Point::new(0, 0)))
        .with(Angle { angle: 0.0 })
        .with(Sprite {
            spritesheet: tank_base_sprite,
            region: Rect::new(0, 0, 32, 32),
        })
        .with(KeyboardControlled {
            speed: 20,
            rotation_speed: 1,
        })
        .with(Velocity {
            speed: 0,
            direction: Angle {angle: 0.0},
        })
        .build();

    // Init the turret
    world
        .create_entity()
        .with(Position(Point::new(0, 0)))
        .with(Angle { angle: 0.0 })
        .with(Sprite {
            spritesheet: tank_turret_sprite,
            region: Rect::new(0, 0, 32, 32),
        })
        .with(KeyboardControlled {
            speed: 20,
            rotation_speed: 1,
        })
        .with(Velocity {
            speed: 0,
            direction: Angle {angle: 0.0},
        })
        .with(AngularVelocity {
            speed: 0,
            rotation: Rotation::Clockwise,
        })
        .with(BulletSpawner {spawning: false, cooldown: 2, cooldown_rem: 0, bullet_speed: 15})
        .build();


    // Init the base
    world
    .create_entity()
    .with(Position(Point::new(10, 0)))
    .with(Angle { angle: 0.0 })
    .with(Sprite {
        spritesheet: tank_base_sprite,
        region: Rect::new(0, 0, 32, 32),
    })
    .with(Velocity {
        speed: 0,
        direction: Angle {angle: 0.0},
    })
    .with(AngularVelocity {
        speed: 2,
        rotation: Rotation::Clockwise,
    })
    .build();

    // Init the turret  
    world
    .create_entity()
    .with(Position(Point::new(10, 0)))
    .with(Angle { angle: 10.0 })
    .with(Sprite {
        spritesheet: tank_turret_sprite,
        region: Rect::new(0, 0, 32, 32),
    })
    .with(AngularVelocity {
        speed: 3,
        rotation: Rotation::Clockwise,
    })
    .with(BulletSpawner {spawning: true, cooldown: 15, cooldown_rem: 0, bullet_speed: 8})
    .build();
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rusty-tanks", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::KeyboardShoot, "KeyboardShoot", &[])
        .with(keyboard::KeyboardMove, "KeyboardMove", &[])
        .with(keyboard::KeyboardRotate, "KeyboardRotate", &[])
        .with(bulletspawner_system::BulletSpawnerSystem, "BulletSpawnerSystem", &["KeyboardShoot", "KeyboardMove", "KeyboardRotate"])
        .with(
            physics::Physics,
            "Physics",
            &["BulletSpawnerSystem", "KeyboardShoot", "KeyboardMove", "KeyboardRotate"],
        )
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    // Initialize resource
    let movement_command: Option<MovementCommand> = None;
    world.insert(movement_command);

    let fire_command: Option<FireCommand> = None;
    world.insert(fire_command);

    let rotation_command: Option<RotationCommand> = None;
    world.insert(rotation_command);

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
        let mut movement_command_one = None;
        let mut rotation_command = None;
        let mut fire_command = None;

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                // move
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    movement_command_one = Some(MovementCommand::Move(Angle {angle: 180.0}));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    movement_command_one = Some(MovementCommand::Move(Angle {angle: 0.0}));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    movement_command_one = Some(MovementCommand::Move(Angle {angle: 270.0}));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movement_command_one = Some(MovementCommand::Move(Angle {angle: 90.0}));
                }

                // rotate
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    repeat: false,
                    ..
                } => {
                    rotation_command = Some(RotationCommand::Move(Rotation::CounterClockwise));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    repeat: false,
                    ..
                } => {
                    rotation_command = Some(RotationCommand::Move(Rotation::Clockwise));
                }

                // fire
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    fire_command = Some(FireCommand::Fire);
                }

                // stop move
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movement_command_one = Some(MovementCommand::Stop);
                }

                // stop rotate
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::E),
                    repeat: false,
                    ..
                } => {
                    rotation_command = Some(RotationCommand::Stop);
                }

                // stop fire
                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    fire_command = Some(FireCommand::Stop);
                }

                _ => {}
            }
        }

        *world.write_resource() = movement_command_one;
        // println!("{:?}", rotation_command);
        *world.write_resource() = rotation_command;
        *world.write_resource() = fire_command;
        

        // Update
        dispatcher.dispatch(&mut world);
        world.maintain();

        // Render
        i = (i + 1) % 255;
        renderer::render(
            &mut canvas,
            Color::RGB(i, 64, 255 - i),
            &textures,
            world.system_data(),
        )?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
