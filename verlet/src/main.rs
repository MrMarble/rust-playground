use std::f32::consts::PI;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Verlet".to_owned(),
        window_width: 1000,
        window_height: 1000,
        ..Default::default()
    }
}

const MAX_OBJECTS: usize = 400;
const SPAWN_RATE: f32 = 0.025;
const SPAWN_SPEED: f32 = 5.0;

#[macroquad::main(window_conf)]
async fn main() {
    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mut solver = Solver::new(3);
    solver.set_constraint(center, screen_height() / 2.0 - 20.0);

    let mut spawn_timer = 0.0;
    loop {
        clear_background(DARKGRAY);

        solver.draw();
        solver.update();

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = vec2(mouse_position().0, mouse_position().1);
            solver.add_object(Object::new(
                mouse_pos,
                rand::gen_range(screen_width() * 0.005, screen_width() * 0.020),
                get_rainbow(get_time() as f32),
            ));
        }

        if solver.objects.len() < MAX_OBJECTS && spawn_timer > SPAWN_RATE {
            let angle = get_time().sin() as f32 + PI * 0.5;
            let mut obj = Object::new(
                vec2(screen_width() / 2.0, 30.0),
                rand::gen_range(screen_width() * 0.005, screen_width() * 0.020),
                get_rainbow(get_time() as f32),
            );
            obj.set_velocity(vec2(angle.cos(), angle.sin()) * SPAWN_SPEED);
            solver.add_object(obj);
            spawn_timer = 0.0;
        }
        spawn_timer += get_frame_time();
        draw_text(&format!("FPS {}", get_fps()), 20.0, 20.0, 30.0, WHITE);
        draw_text(
            &format!("Objects {}", solver.objects.len()),
            20.0,
            40.0,
            30.0,
            WHITE,
        );
        next_frame().await
    }
}

fn get_rainbow(t: f32) -> Color {
    let r = t.sin();
    let g = (t + PI * 2.0 / 3.0).sin();
    let b = (t + PI * 4.0 / 3.0).sin();
    Color::new(r, g, b, 1.0)
}

struct Solver {
    objects: Vec<Object>,
    gravity: Vec2,
    constraint_center: Vec2,
    constraint_radius: f32,
    sub_steps: usize,
}

impl Solver {
    fn new(sub_steps: usize) -> Self {
        Self {
            objects: vec![],
            gravity: vec2(0.0, 1000.0),
            constraint_center: vec2(0.0, 0.0),
            constraint_radius: 0.0,
            sub_steps,
        }
    }

    fn draw(&self) {
        draw_poly(
            self.constraint_center.x,
            self.constraint_center.y,
            40,
            self.constraint_radius,
            0.0,
            BLACK,
        );

        for object in &self.objects {
            object.draw();
        }
    }

    fn set_constraint(&mut self, center: Vec2, radius: f32) {
        self.constraint_center = center;
        self.constraint_radius = radius;
    }

    fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    fn update(&mut self) {
        let step_time = get_frame_time() / self.sub_steps as f32;
        for _ in 0..self.sub_steps {
            self.apply_gravity();
            self.solve_collisions();
            self.update_objects(step_time);
            self.apply_constraint();
        }
    }

    fn apply_gravity(&mut self) {
        for object in &mut self.objects {
            object.accelerate(self.gravity);
        }
    }

    fn apply_constraint(&mut self) {
        for object in &mut self.objects {
            let distance = object.position.distance(self.constraint_center);
            if distance > self.constraint_radius - object.radius {
                let direction = (object.position - self.constraint_center).normalize();
                let displacement = distance - self.constraint_radius + object.radius;
                object.position -= direction * displacement;
            }
        }
    }

    fn solve_collisions(&mut self) {
        let object_count = self.objects.len();
        for i in 0..object_count {
            for j in i + 1..object_count {
                let distance = self.objects[i].position.distance(self.objects[j].position);
                if distance < self.objects[i].radius + self.objects[j].radius {
                    let direction =
                        (self.objects[i].position - self.objects[j].position).normalize();
                    let displacement = distance - self.objects[i].radius - self.objects[j].radius;
                    let new_pos = direction * displacement * 0.5;
                    self.objects[i].position -= new_pos;
                    self.objects[j].position += new_pos;
                }
            }
        }
    }

    fn update_objects(&mut self, dt: f32) {
        for object in &mut self.objects {
            object.update(dt);
        }
    }
}

struct Object {
    position: Vec2,
    old_position: Vec2,
    acceleration: Vec2,
    radius: f32,
    color: Color,
}

impl Object {
    fn new(position: Vec2, radius: f32, color: Color) -> Self {
        Self {
            position,
            old_position: position,
            radius,
            acceleration: vec2(0.0, 0.0),
            color,
        }
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }

    fn accelerate(&mut self, acceleration: Vec2) {
        self.acceleration += acceleration;
    }

    fn set_velocity(&mut self, velocity: Vec2) {
        self.old_position = self.position - velocity;
    }

    fn update(&mut self, dt: f32) {
        // compute displacement
        let displacement = self.position - self.old_position;
        // store the current position
        self.old_position = self.position;
        // update the position (verlet integration)
        self.position += displacement + self.acceleration * (dt * dt);
        // reset acceleration
        self.acceleration = vec2(0.0, 0.0);
    }
}
