use crate::components::Angle;
use crate::components::AngularVelocity;
use crate::components::BulletSpawner;
use crate::components::Position;
use crate::components::Rotation;
use crate::components::Sprite;
use crate::components::Velocity;
use sdl2::rect::{Point, Rect};
use specs::join::Join;
use specs::storage::WriteStorage;
use specs::Entities;
use specs::LazyUpdate;
use specs::Read;
use specs::ReadStorage;
use specs::System;
pub struct BulletSpawnerSystem;

impl<'a> System<'a> for BulletSpawnerSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Angle>,
        WriteStorage<'a, BulletSpawner>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let entities = data.3;
        let updater = data.4;
        for (pos, angle, spawner) in (&data.0, &data.1, &mut data.2).join() {
            match spawner {
                BulletSpawner {
                    spawning: true,
                    cooldown_rem: 0,
                    cooldown: _,
                    bullet_speed: _,
                } => {
                    spawner.cooldown_rem = spawner.cooldown;

                    let bullet = entities.create();
                    updater.insert(
                        bullet,
                        Velocity {
                            speed: spawner.bullet_speed,
                            direction: Angle { angle: angle.angle },
                        },
                    );
                    updater.insert(bullet, Position(Point::new(pos.0.x, pos.0.y)));
                    updater.insert(bullet, Angle { angle: angle.angle });
                    updater.insert(
                        bullet,
                        AngularVelocity {
                            speed: 2.0,
                            rotation: Rotation::Clockwise,
                        },
                    );
                    updater.insert(
                        bullet,
                        Sprite {
                            spritesheet: 0,
                            region: Rect::new(0, 0, 32, 32),
                        },
                    );
                }
                BulletSpawner {
                    spawning: _,
                    cooldown_rem: 1..=u32::MAX,
                    cooldown: _,
                    bullet_speed: _,
                } => {
                    spawner.cooldown_rem -= 1;
                }
                _ => {}
            }
        }
    }
}
