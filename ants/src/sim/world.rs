use macroquad::{
    prelude::{Vec2, BLACK, RED, WHITE},
    shapes::draw_rectangle_lines,
    texture::{draw_texture, Image, Texture2D},
    window::screen_width,
};

use crate::{Config, MAX_MARKER_PER_CELL};

use super::{Food, Marker, MarkerType};

pub struct Grid<T> {
    width: usize,
    height: usize,
    cell_size: usize,
    cells: Vec<Vec<T>>,
}

impl<T: Clone> Grid<T> {
    fn new(_width: usize, _height: usize, cell_size: usize) -> Self {
        let width = _width / cell_size;
        let height = _height / cell_size;

        Self {
            width,
            height,
            cells: vec![vec![]; width * height],
            cell_size,
        }
    }

    fn check_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_cell_coords(&self, pos: Vec2) -> (usize, usize) {
        let x = pos.x as usize / self.cell_size;
        let y = pos.y as usize / self.cell_size;

        (x, y)
    }

    fn add(&mut self, pos: Vec2, value: T) {
        let (x, y) = self.get_cell_coords(pos);
        if self.check_bounds(x, y) {
            let index = self.get_index(x, y);
            if self.cells[index].len() < MAX_MARKER_PER_CELL {
                self.cells[index].push(value);
            }
        }
    }

    pub fn get_all_at(&self, pos: Vec2) -> Vec<T> {
        let (x, y) = self.get_cell_coords(pos);

        let mut result = vec![];

        for dx in 1..=3 {
            for dy in 1..=3 {
                let x = (x as i32 + (dx - 2)) as usize;
                let y = (y as i32 + (dy - 2)) as usize;
                if self.check_bounds(x, y) {
                    let index = self.get_index(x, y);
                    for cell in self.cells[index].iter() {
                        result.push(cell.clone());
                    }
                }
            }
        }
        return result;
    }

    pub fn get_all_at_mut(&mut self, pos: Vec2) -> Vec<&mut T> {
        let (x, y) = self.get_cell_coords(pos);
        if self.check_bounds(x, y) {
            let index = self.get_index(x, y);
            self.cells[index].iter_mut().collect()
        } else {
            vec![]
        }
    }
}

pub struct World {
    grid_home_markers: Grid<Marker>,
    grid_food_markers: Grid<Marker>,
    pub grid_food: Grid<Food>,
    texture: Texture2D,
    img: Image,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let img = Image::gen_image_color(width as u16, height as u16, BLACK);
        Self {
            grid_home_markers: Grid::new(width, height, 58),
            grid_food_markers: Grid::new(width, height, 58),
            grid_food: Grid::new(width, height, 5),
            texture: Texture2D::from_image(&img),
            img,
        }
    }

    pub fn get_grid(&self, marker_type: MarkerType) -> &Grid<Marker> {
        match marker_type {
            MarkerType::ToFood => &self.grid_food_markers,
            MarkerType::ToHome => &self.grid_home_markers,
        }
    }

    fn get_grid_mut(&mut self, marker_type: MarkerType) -> &mut Grid<Marker> {
        match marker_type {
            MarkerType::ToFood => &mut self.grid_food_markers,
            MarkerType::ToHome => &mut self.grid_home_markers,
        }
    }

    pub fn add_marker(&mut self, marker: Marker) {
        let grid = self.get_grid_mut(marker.marker_type);
        grid.add(marker.position, marker);
    }

    pub fn add_food(&mut self, food: Food) {
        self.grid_food.add(food.position, food);
        self.add_marker(Marker::new(
            food.position,
            MarkerType::ToFood,
            10000.0,
            false,
        ));
    }

    fn remove_expired_markers(&mut self) {
        for cell_list in &mut self.grid_home_markers.cells {
            cell_list.retain(|cell| cell.intensity > 0.0);
        }

        for cell_list in &mut self.grid_food_markers.cells {
            cell_list.retain(|cell| cell.intensity > 0.0);
        }
    }

    fn remove_expired_food(&mut self) {
        for cell_list in &mut self.grid_food.cells {
            cell_list.retain(|cell| !cell.is_empty());
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.remove_expired_markers();
        self.remove_expired_food();

        for cell_list in &mut self.grid_home_markers.cells {
            for cell in cell_list {
                cell.update(dt);
            }
        }

        for cell_list in &mut self.grid_food_markers.cells {
            for cell in cell_list {
                cell.update(dt);
            }
        }
    }

    pub fn draw(&mut self, cfg: &Config) {
        if cfg.draw_markers {
            for cell_list in &self.grid_home_markers.cells {
                for cell in cell_list {
                    cell.draw(&mut self.img);
                }
            }

            for cell_list in &self.grid_food_markers.cells {
                for cell in cell_list {
                    cell.draw(&mut self.img);
                }
            }

            self.texture.update(&self.img);
            draw_texture(self.texture, 0., 0., WHITE);
        }

        for cell_list in &self.grid_food.cells {
            for cell in cell_list {
                cell.draw();
            }
        }

        if cfg.draw_grid {
            for i in 0..self.grid_food_markers.cells.len() {
                // draw grid
                let x = i % (screen_width() / self.grid_food_markers.cell_size as f32) as usize;
                let y = i / (screen_width() / self.grid_food_markers.cell_size as f32) as usize;
                draw_rectangle_lines(
                    (x * self.grid_food_markers.cell_size) as f32,
                    (y * self.grid_food_markers.cell_size) as f32,
                    self.grid_food_markers.cell_size as f32,
                    self.grid_food_markers.cell_size as f32,
                    2.,
                    RED,
                );
            }
        }
    }
}
