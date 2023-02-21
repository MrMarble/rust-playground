use std::f32::consts::PI;

use macroquad::prelude::*;
use sim::*;

mod sim;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ants".to_owned(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

pub struct Config {
    draw_grid: bool,
    draw_ants: bool,
    draw_markers: bool,
}

const MAX_RESERVE: f32 = 2000.0;
const DIRECTION_UPDATE_PERIOD: f32 = 0.125;
const MOVE_SPEED: f32 = 50.0;
const MARKER_DETECTION_RADIUS: f32 = 40.0;
const MARKER_PERIOD: f32 = 0.25;
const DIRECTION_NOISE: f32 = PI * 0.1;
const ROTATION_SPEED: f32 = 10.0;
const MAX_MARKER_PER_CELL: usize = 1024;
const MAX_ANTS: usize = 512;
#[macroquad::main(window_conf)]
async fn main() {
    println!("{} {}", screen_width(), screen_height());

    let ant_texture = load_texture("assets/ant.png").await.unwrap();
    let mut colony = Colony::new(vec2(screen_width() * 0.3, screen_height() * 0.6));
    let mut world = World::new(screen_width() as usize, screen_height() as usize);
    world.add_marker(Marker::new(colony.position, MarkerType::ToHome, 10.0, true));

    world.add_food(Food::new(
        vec2(screen_width() * 0.5 + 200., screen_height() * 0.5 - 10.),
        4.,
        100.,
    ));

    let mut cfg = Config {
        draw_grid: false,
        draw_ants: true,
        draw_markers: true,
    };

    loop {
        clear_background(BLACK);

        //let render_timer = SystemTime::now();

        world.draw(&cfg);
        colony.draw(ant_texture, &cfg);
        /*draw_text(
            &format!(
                "render: {:.3}ms",
                render_timer.elapsed().unwrap().as_millis()
            ),
            20.0,
            50.0,
            20.0,
            WHITE,
        );
        let update_timer = SystemTime::now();*/
        colony.update(0.016, &mut world);
        world.update(0.016);
        /*draw_text(
            &format!(
                "update: {:.3}ms",
                update_timer.elapsed().unwrap().as_millis()
            ),
            20.0,
            65.0,
            20.0,
            WHITE,
        );*/
        draw_text(&format!("FPS {}", get_fps()), 20.0, 20.0, 20.0, WHITE);
        draw_text(
            &format!("frame: {:.3}ms", get_frame_time()),
            20.0,
            35.0,
            20.0,
            WHITE,
        );

        if is_key_pressed(KeyCode::G) {
            cfg.draw_grid = !cfg.draw_grid;
        }
        if is_key_pressed(KeyCode::A) {
            cfg.draw_ants = !cfg.draw_ants;
        }
        if is_key_pressed(KeyCode::M) {
            cfg.draw_markers = !cfg.draw_markers;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            world.add_food(Food::new(vec2(mouse_pos.0, mouse_pos.1), 4., 100.));
        }

        next_frame().await
    }
}
