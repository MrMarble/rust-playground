use macroquad::{
    prelude::{Vec2, GREEN},
    shapes::draw_circle,
};

#[derive(Clone, Copy)]
pub struct Food {
    pub position: Vec2,
    pub radius: f32,
    quantity: f32,
}

impl Food {
    pub fn new(position: Vec2, radius: f32, quantity: f32) -> Self {
        Self {
            position,
            radius,
            quantity,
        }
        //TODO: add marker
    }

    pub fn is_empty(&self) -> bool {
        self.quantity <= 0.0
    }

    pub fn pick(&mut self) {
        self.quantity -= 1.0;
        if self.is_empty() {
            //TODO: handle marker
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, GREEN);
    }
}
