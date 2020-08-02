use core::f32::consts::PI;
use crate::components::Angle;
use crate::components::AngularVelocity;
use crate::components::Direction;
use crate::components::Position;
use crate::components::Velocity;
use crate::components::Rotation;
use specs::storage::WriteStorage;

use specs::ReadStorage;
use specs::System;

use specs::join::Join;
pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>, WriteStorage<'a, Angle>, ReadStorage<'a, AngularVelocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        //TODO: This code can be made nicer and more idiomatic using more pattern matching.
        // Look up "rust irrefutable patterns" and use them here.
        for (pos, vel) in (&mut data.0, &data.1).join() {

            let x = vel.speed as f32 * (vel.direction.angle * (PI/180.0)).cos();
            let y = vel.speed as f32 * (vel.direction.angle * (PI/180.0)).sin();
            pos.0 = pos.0.offset(x as i32, y as i32);
        }

        for (angle, angular_vel) in (&mut data.2, &data.3).join() {
            match angular_vel.rotation {
                Rotation::Clockwise => {
                    angle.angle = (angle.angle + angular_vel.speed as f32) % 360.0;
                }
                Rotation::CounterClockwise => {
                    angle.angle = (angle.angle - angular_vel.speed as f32) % 360.0;
                }
            }
        }
    }
}