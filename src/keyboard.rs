use crate::components::KeyboardControlled;
use crate::components::Velocity;
use specs::join::Join;
use specs::storage::WriteStorage;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;

use super::MovementCommand;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        //TODO: This code can be made nicer and more idiomatic using more pattern matching.
        // Look up "rust irrefutable patterns" and use them here.
        let movement_command = match &*data.0 {
            Some(movement_command) => movement_command,
            None => return, // no change
        };

        for (control, vel) in (&data.1, &mut data.2).join() {
            match movement_command {
                &MovementCommand::Move(direction) => {
                    vel.speed = control.speed;
                    vel.direction = direction;
                }
                MovementCommand::Stop => vel.speed = 0,
            }
        }
    }
}
