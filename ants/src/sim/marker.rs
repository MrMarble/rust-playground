use macroquad::{
    prelude::{Color, Vec2},
    texture::Image,
};

#[derive(Clone, Copy, Debug)]
pub enum MarkerType {
    ToFood,
    ToHome,
}

#[derive(Clone, Copy)]
pub struct Marker {
    pub position: Vec2,
    pub marker_type: MarkerType,
    pub intensity: f32,
    permanent: bool,
    initial_intensity: f32,
}

impl Marker {
    pub fn new(position: Vec2, marker_type: MarkerType, intensity: f32, permanent: bool) -> Self {
        Self {
            position,
            marker_type,
            intensity,
            initial_intensity: intensity,
            permanent,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.permanent {
            self.intensity -= dt * 1.0;
        }
    }

    pub fn draw(&self, img: &mut Image) {
        let color = match self.marker_type {
            MarkerType::ToFood => {
                Color::new(0.00, 0.89, 0.19, self.intensity / self.initial_intensity)
            }
            MarkerType::ToHome => {
                Color::new(0.00, 0.47, 0.95, self.intensity / self.initial_intensity)
            }
        };
        img.set_pixel(self.position.x as u32, self.position.y as u32, color);
    }
}
