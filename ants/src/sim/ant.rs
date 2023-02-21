use std::f32::consts::PI;

use macroquad::{
    prelude::{vec2, Vec2, RED},
    rand,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
    window::{screen_height, screen_width},
};

use crate::{
    DIRECTION_NOISE, DIRECTION_UPDATE_PERIOD, MARKER_DETECTION_RADIUS, MARKER_PERIOD, MAX_RESERVE,
    MOVE_SPEED, ROTATION_SPEED,
};

use super::{Marker, MarkerType, World};

#[derive(Clone, Copy)]
pub struct Ant {
    position: Vec2,
    direction: Direction,
    last_direction_update: f32,
    last_marker: f32,
    phase: MarkerType,
    reserve: f32,
}

impl Ant {
    pub fn new(position: Vec2, rotation: f32) -> Self {
        Self {
            position,
            direction: Direction::new(rotation, ROTATION_SPEED),
            last_direction_update: rand::gen_range(0., 100.0) * 0.01 * DIRECTION_UPDATE_PERIOD,
            last_marker: rand::gen_range(0., 100.0) * MARKER_PERIOD * 0.01,
            phase: MarkerType::ToFood,
            reserve: MAX_RESERVE,
        }
    }

    pub fn update(&mut self, dt: f32, world: &mut World) {
        self.update_position(dt);

        match self.phase {
            MarkerType::ToFood => self.check_food(world),
            MarkerType::ToHome => {}
        }

        self.last_direction_update += dt;
        if self.last_direction_update > DIRECTION_UPDATE_PERIOD {
            self.find_marker(world);
            self.direction += rand::gen_range(-DIRECTION_NOISE, DIRECTION_NOISE);
            self.last_direction_update = 0.0;
        }

        self.last_marker += dt;
        if self.last_marker > MARKER_PERIOD {
            self.add_marker(world);
        }

        self.direction.update(dt);
    }

    fn add_marker(&mut self, world: &mut World) {
        if self.reserve > 1.0 {
            let marker_type = match self.phase {
                MarkerType::ToFood => MarkerType::ToHome,
                MarkerType::ToHome => MarkerType::ToFood,
            };

            world.add_marker(Marker::new(
                self.position,
                marker_type,
                self.reserve * 0.02,
                false,
            ));
            self.reserve *= 0.98;
        }

        self.last_marker = 0.0;
    }

    fn update_position(&mut self, dt: f32) {
        self.position += (dt * MOVE_SPEED) * self.direction.vec;

        // check out of bounds
        if self.position.x < 0.0 {
            self.position.x = 0.0;
            self.direction.add_now(PI);
        } else if self.position.x > screen_width() {
            self.position.x = screen_width();
            self.direction.add_now(PI);
        }

        if self.position.y < 0.0 {
            self.position.y = 0.0;
            self.direction.add_now(PI);
        } else if self.position.y > screen_height() {
            self.position.y = screen_height();
            self.direction.add_now(PI);
        }
    }

    fn find_marker(&mut self, world: &mut World) {
        let markers = world.get_grid(self.phase).get_all_at(self.position);

        let mut total_intensity = 0.0f32;
        let mut point = Vec2::new(0.0, 0.0);

        let dir = self.direction.vec;

        for marker in markers.iter() {
            let to_marker = marker.position - self.position;
            let lenght = to_marker.length();
            if lenght < MARKER_DETECTION_RADIUS {
                if to_marker.dot(dir) > 0.0 {
                    total_intensity += marker.intensity;
                    point += marker.position * marker.intensity;
                }
            }
        }

        if total_intensity > 0.0 {
            let dst = point / total_intensity - self.position;
            let angle = (dst.x / dst.length()).acos();

            self.direction.target_angle = if dst.y > 0.0 { angle } else { -angle };
            self.direction.update_target_vec();
        }
    }

    fn check_food(&mut self, world: &mut World) {
        let mut food_spots = world.grid_food.get_all_at_mut(self.position);
        for food_spot in food_spots.iter_mut() {
            if self.position.distance(food_spot.position) < food_spot.radius {
                self.phase = MarkerType::ToHome;
                self.reserve = MAX_RESERVE;
                self.direction.add_now(PI);
                food_spot.pick();
                return;
            }
        }
    }

    pub fn check_colony(&mut self, colony_position: Vec2) {
        if self.position.distance(colony_position) < 10.0 {
            match self.phase {
                MarkerType::ToFood => {}
                MarkerType::ToHome => {
                    self.direction.add_now(PI);
                    self.phase = MarkerType::ToFood;
                }
            }
            self.reserve = MAX_RESERVE;
        }
    }

    pub fn draw(&self, texture: Texture2D) {
        /*draw_circle(
            self.position.x,
            self.position.y,
            MARKER_DETECTION_RADIUS,
            Color::new(0.99, 0.98, 0.00, 0.1),
        );*/
        draw_texture_ex(
            texture,
            self.position.x - 5.0,
            self.position.y - 7.5,
            RED,
            DrawTextureParams {
                dest_size: Some(vec2(5.0, 7.0)),
                source: None,
                rotation: self.direction.angle + (PI / 2.0),
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }
}

#[derive(Clone, Copy)]
struct Direction {
    angle: f32,
    target_angle: f32,
    rotation_speed: f32,
    vec: Vec2,
    target_vec: Vec2,
}

impl std::ops::AddAssign<f32> for Direction {
    fn add_assign(&mut self, rhs: f32) {
        self.target_angle += rhs;
        self.update_target_vec();
    }
}

impl std::ops::Add<f32> for Direction {
    type Output = Self;

    fn add(mut self, rhs: f32) -> Self::Output {
        self.target_angle += rhs;
        self.update_target_vec();
        self
    }
}

impl Direction {
    fn new(angle: f32, rotation_speed: f32) -> Self {
        let mut dir = Self {
            angle,
            target_angle: angle,
            rotation_speed,
            vec: vec2(angle.cos(), angle.sin()),
            target_vec: vec2(angle.cos(), angle.sin()),
        };

        dir.update_vec();
        dir.target_vec = dir.vec;
        dir
    }

    fn update(&mut self, dt: f32) {
        self.update_vec();

        //let dir_nrm = self.vec.normalize();
        let dir_nrm = vec2(-self.vec.y, self.vec.x);
        let dir_delta = self.target_vec.dot(dir_nrm);
        self.angle += self.rotation_speed * dir_delta * dt;
    }

    fn update_vec(&mut self) {
        self.vec = vec2(self.angle.cos(), self.angle.sin());
    }

    fn update_target_vec(&mut self) {
        self.target_vec = vec2(self.target_angle.cos(), self.target_angle.sin());
    }

    fn add_now(&mut self, angle: f32) {
        self.target_angle += angle;
        self.update_target_vec();
        self.angle = self.target_angle;
        self.update_vec();
    }
}
