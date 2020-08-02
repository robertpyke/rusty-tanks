use crate::components::AngularVelocity;
use crate::components::BulletSpawner;
use crate::components::KeyboardControlled;
use crate::components::Velocity;
use crate::FireCommand;
use specs::join::Join;
use specs::storage::WriteStorage;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;

use super::MovementCommand;
use super::RotationCommand;

pub struct KeyboardMove;

impl<'a> System<'a> for KeyboardMove {
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
                MovementCommand::Stop => vel.speed = 0.0,
            }
        }
    }
}

pub struct KeyboardRotate;

impl<'a> System<'a> for KeyboardRotate {
    type SystemData = (
        ReadExpect<'a, Option<RotationCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, AngularVelocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        //TODO: This code can be made nicer and more idiomatic using more pattern matching.
        // Look up "rust irrefutable patterns" and use them here.
        let rotation_command = match &*data.0 {
            Some(rotation_command) => rotation_command,
            None => return, // no change
        };
        for (control, agular_velocity) in (&data.1, &mut data.2).join() {
            match rotation_command {
                &RotationCommand::Move(rotation) => {
                    agular_velocity.speed = control.rotation_speed;
                    agular_velocity.rotation = rotation;
                }
                RotationCommand::Stop => agular_velocity.speed = 0.0,
            }
        }
    }
}

pub struct KeyboardShoot;

impl<'a> System<'a> for KeyboardShoot {
    type SystemData = (
        ReadExpect<'a, Option<FireCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, BulletSpawner>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        //TODO: This code can be made nicer and more idiomatic using more pattern matching.
        // Look up "rust irrefutable patterns" and use them here.
        let fire_command = match &*data.0 {
            Some(fire_command) => fire_command,
            None => return, // no change
        };

        for (spawner, _kc) in (&mut data.2, &data.1).join() {
            match fire_command {
                &FireCommand::Fire => {
                    spawner.spawning = true;
                }
                FireCommand::Stop => {
                    spawner.spawning = false;
                }
            }
        }
    }
}
