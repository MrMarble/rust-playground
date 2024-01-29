use std::{
    ops::{Add, AddAssign, Mul, Sub},
    time::Instant,
};

use macroquad::prelude::*;

mod perlin;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn window_conf() -> Conf {
    Conf {
        window_title: "Island".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut p = perlin::Perlin::new();

    let mut terrain = generate_terrain(&p);
    let mut img = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&img);

    // Colorize
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let n = terrain[x][y];
            //let color = Color::new(n, n, n, 1.0);
            img.set_pixel(x as u32, y as u32, colorize(n));
        }
    }

    // Update texture
    texture.update(&img);

    loop {
        clear_background(BLACK);

        // Update shadows
        if is_mouse_button_pressed(MouseButton::Left) {
            let start = Instant::now();
            let (mx, my) = mouse_position();
            let mut shadow_img: Image = Image::from(img.clone());
            draw_shadows(&mut shadow_img, &terrain, vec3(mx, my, 1.0));
            texture.update(&shadow_img);

            println!("shadow time: {:?}", start.elapsed());
        }

        if is_key_pressed(KeyCode::Space) {
            let start = Instant::now();
            p.reseed();
            terrain = generate_terrain(&p);
            // Colorize
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let n = terrain[x][y];
                    //let color = Color::new(n, n, n, 1.0);
                    img.set_pixel(x as u32, y as u32, colorize(n));
                }
            }

            // Update texture
            texture.update(&img);
            println!("island time: {:?}", start.elapsed());
        }

        // Draw to screen
        draw_texture(&texture, 0.0, 0.0, WHITE);
        // Draw FPS counter
        draw_text(&format!("FPS {}", get_fps()), 20.0, 20.0, 20.0, WHITE);
        draw_text(
            &format!("frame: {:.3}ms", get_frame_time()),
            20.0,
            35.0,
            20.0,
            WHITE,
        );
        next_frame().await
    }
}

fn generate_terrain(p: &perlin::Perlin) -> Vec<Vec<f32>> {
    let mut terrain: Vec<Vec<f32>> = vec![vec![0.0; HEIGHT]; WIDTH];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let n = p.noise2d(x as f32 * 0.01, y as f32 * 0.01);
            terrain[x][y] = (n * island_mod(x as f32, y as f32)).max(0.3)
        }
    }
    terrain
}

fn island_mod(x: f32, y: f32) -> f32 {
    let max_dist = (WIDTH as f32 / 2.0 as f32).powi(2);

    let dx = WIDTH as f32 / 2.0 - x;
    let dy = HEIGHT as f32 / 2.0 - y;

    let dist_square = dx.powi(2) + dy.powi(2);
    map(dist_square, 0.0, max_dist, 1.0, 0.0)
}

fn map(value: f32, min1: f32, max1: f32, min2: f32, max2: f32) -> f32 {
    min2 + (value - min1) * (max2 - min2) / (max1 - min1)
}

fn colorize(n: f32) -> Color {
    match n {
        n if n <= 0.3 => Color::from_rgba(98, 166, 169, 255),
        n if n <= 0.4 => Color::from_rgba(214, 182, 158, 255),
        n if n <= 0.5 => Color::from_rgba(152, 173, 90, 255),
        n if n <= 0.6 => Color::from_rgba(101, 133, 65, 255),
        n if n <= 0.7 => Color::from_rgba(71, 118, 69, 255),
        n if n <= 0.8 => Color::from_rgba(109, 118, 135, 255),
        n if n <= 0.9 => Color::from_rgba(132, 141, 154, 255),
        _ => Color::from_rgba(210, 224, 222, 255),
    }
}

fn draw_shadows(img: &mut Image, terrain: &Vec<Vec<f32>>, sun: Vec3) {
    // Colorize
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let h = terrain[x][y];

            let mut p = vec3(x as f32, y as f32, h);

            let step_dir = sun.sub(p).mul(1.0 / 200.0);

            let mut in_shadow = false;
            for _ in 0..200 {
                p.add_assign(step_dir);
                let height = terrain[p.x as usize][p.y as usize];
                if height > p.z {
                    in_shadow = true;
                    break;
                }

                if p.z >= 1.0 {
                    break;
                }
            }

            if in_shadow {
                let c = shade(colorize(h));
                img.set_pixel(x as u32, y as u32, c);
            }
        }
    }
}

fn shade(color: Color) -> Color {
    return mix(color, Color::from_hex(0x000), 0.5);
}

fn mix(color_a: Color, color_b: Color, amount: f32) -> Color {
    let r = color_a.r * (1.0 - amount) + color_b.r * amount;
    let g = color_a.g * (1.0 - amount) + color_b.g * amount;
    let b = color_a.b * (1.0 - amount) + color_b.b * amount;
    let a = color_a.a * (1.0 - amount) + color_b.a * amount;
    Color::new(r, g, b, a)
}
