use std::f32::consts::PI;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ants".to_owned(),
        window_width: 1000,
        window_height: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let ant_texture = load_texture("ant.png").await.unwrap();
    let mut colony = Colony::new(vec2(screen_width() * 0.5, screen_height() * 0.5));

    loop {
        clear_background(BLACK);

        colony.draw(ant_texture);
        colony.update(0.016);

        draw_text(&format!("FPS {}", get_fps()), 20.0, 20.0, 20.0, WHITE);
        next_frame().await
    }
}

struct Colony {
    position: Vec2,
    ants: [Ant; 100],
}

impl Colony {
    fn new(position: Vec2) -> Self {
        let mut ants = [Ant::new(position, 0.0); 100];
        // TODO: Randomize rotation
        for ant in ants.iter_mut() {
            *ant = Ant::new(position, rand::gen_range(0.0, 2.0 * PI));
        }
        Self { position, ants }
    }

    fn update(&mut self, dt: f32) {
        for ant in self.ants.iter_mut() {
            ant.update(dt);
        }
    }

    fn draw(&self, texture: Texture2D) {
        draw_circle(self.position.x, self.position.y, 10.0, DARKBLUE);
        for ant in self.ants.iter() {
            ant.draw(texture);
        }
    }
}

#[derive(Clone, Copy)]
struct Ant {
    position: Vec2,
    direction: Direction,
    last_direction_update: f32,
}

impl Ant {
    fn new(position: Vec2, rotation: f32) -> Self {
        Self {
            position,
            direction: Direction::new(rotation, 10.0),
            last_direction_update: rand::gen_range(0.01 * 0.125, 100.0),
        }
    }

    fn update(&mut self, dt: f32) {
        self.update_position(dt);

        self.last_direction_update += dt;
        if self.last_direction_update > 0.125 {
            self.direction += rand::gen_range(0., PI * 0.1);
            self.last_direction_update = 0.0;
        }

        self.direction.update(dt);
    }

    fn update_position(&mut self, dt: f32) {
        self.position += (dt * 50.0) * self.direction.vec;

        // check out of bounds
        if self.position.x < 0.0 {
            self.position.x = screen_width();
        } else if self.position.x > screen_width() {
            self.position.x = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = screen_height();
        } else if self.position.y > screen_height() {
            self.position.y = 0.0;
        }
    }

    fn draw(&self, texture: Texture2D) {
        draw_texture_ex(
            texture,
            self.position.x - 5.0,
            self.position.y - 7.5,
            RED,
            DrawTextureParams {
                dest_size: Some(vec2(10.0, 15.0)),
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

        let dir_nrm = self.vec.normalize();
        let dir_delta = self.target_vec.dot(dir_nrm);
        self.angle += self.rotation_speed * dir_delta * dt;
    }

    fn update_vec(&mut self) {
        self.vec = vec2(self.angle.cos(), self.angle.sin());
    }

    fn update_target_vec(&mut self) {
        self.target_vec.x += self.target_angle.cos();
        self.target_vec.y += self.target_angle.sin();
    }
}
