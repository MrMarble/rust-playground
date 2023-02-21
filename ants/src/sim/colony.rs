use std::f32::consts::PI;

use macroquad::{
    prelude::{Vec2, DARKBLUE},
    rand,
    shapes::draw_circle,
    texture::Texture2D,
};

use crate::{Config, MAX_ANTS};

use super::{Ant, World};

pub struct Colony {
    pub position: Vec2,
    ants: [Ant; MAX_ANTS],
}

impl Colony {
    pub fn new(position: Vec2) -> Self {
        let mut ants = [Ant::new(position, 0.0); MAX_ANTS];
        for ant in ants.iter_mut() {
            *ant = Ant::new(position, rand::gen_range(2.0 * -PI, 2.0 * PI));
        }
        Self { position, ants }
    }

    pub fn update(&mut self, dt: f32, world: &mut World) {
        for ant in self.ants.iter_mut() {
            ant.update(dt, world);
        }

        for ant in self.ants.iter_mut() {
            ant.check_colony(self.position);
        }
    }

    pub fn draw(&self, texture: Texture2D, cfg: &Config) {
        draw_circle(self.position.x, self.position.y, 10.0, DARKBLUE);
        if cfg.draw_ants {
            for ant in self.ants.iter() {
                ant.draw(texture);
            }
        }
    }
}
