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
        use self::Direction::*;
        //TODO: This code can be made nicer and more idiomatic using more pattern matching.
        // Look up "rust irrefutable patterns" and use them here.
        for (pos, vel) in (&mut data.0, &data.1).join() {
            match vel.direction {
                Left => {
                    pos.0 = pos.0.offset(-vel.speed, 0);
                }
                Right => {
                    pos.0 = pos.0.offset(vel.speed, 0);
                }
                Up => {
                    pos.0 = pos.0.offset(0, -vel.speed);
                }
                Down => {
                    pos.0 = pos.0.offset(0, vel.speed);
                }
            }
        }

        for (angle, angular_vel) in (&mut data.2, &data.3).join() {
            match angular_vel.rotation {
                Rotation::Clockwise => {
                    angle.angle += angular_vel.speed;
                }
                Rotation::CounterClockwise => {
                    angle.angle -= angular_vel.speed;
                }
            }
        }
    }
}
