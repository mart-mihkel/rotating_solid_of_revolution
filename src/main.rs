use std::f32::consts::TAU;

use sfml::graphics::{RenderWindow, Color, RenderTarget, PrimitiveType, Vertex, RenderStates};
use sfml::window::{Style, Event, Key};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 400;

const X_ORIGIN: f32 = SCREEN_WIDTH as f32 / 2.;
const Y_ORIGIN: f32 = SCREEN_HEIGHT as f32 / 2.;

const CROSS_SECTION_RADIUS: f32 = 10.;
const REVOLUTION_RADIUS: f32 = 15.;

const OBJECT_DISTANCE: f32 = CROSS_SECTION_RADIUS + REVOLUTION_RADIUS + 5.;
const PROJECTION_DISTANCE: f32 = SCREEN_WIDTH as f32 * OBJECT_DISTANCE * 3. / (8. * (CROSS_SECTION_RADIUS * REVOLUTION_RADIUS));

const VERTEX_COUNT: u32 = 2_u32.pow(12);

fn main() {
    let mut window = RenderWindow::new(
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        "Rotating solid of revolution",
        Style::DEFAULT,
        &Default::default(),
    );

    window.set_framerate_limit(60);

    let mut vertices = [Vertex::default(); VERTEX_COUNT as usize];

    let (mut y_rotation_angle, mut z_rotation_angle): (f32, f32) = (0.0, 0.0);
    while window.is_open() {
        // event loop
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                Event::KeyPressed { code: Key::P, .. } => {
                    while let Some(event) = window.wait_event() {
                        if let Event::KeyReleased { code: Key::P, .. } = event {
                            break;
                        }
                    }
                },
                _ => {}
            }
        }

        // rendering
        window.clear(Color::BLACK);

        let (y_rotation_sin, y_rotation_cos) = y_rotation_angle.sin_cos();
        let (z_rotation_sin, z_rotation_cos) = z_rotation_angle.sin_cos();

        // solid loop
        let mut vertex_i = 0;
        let mut solid_angle = 0.0;
        while solid_angle < TAU {
            let (solid_sin, solid_cos) = solid_angle.sin_cos();

            // revolution loop
            let mut revolution_angle = 0.0;
            while revolution_angle < TAU {
                let (revolution_sin, revolution_cos) = revolution_angle.sin_cos();

                let v0 = REVOLUTION_RADIUS + CROSS_SECTION_RADIUS * solid_cos;
                let v1 = CROSS_SECTION_RADIUS * y_rotation_cos * solid_sin;
                let (x, y, z) = (
                    v0 * (z_rotation_cos * revolution_cos + y_rotation_sin * z_rotation_sin * revolution_sin) - v1 * z_rotation_sin,
                    v0 * (revolution_cos * y_rotation_sin - z_rotation_cos * y_rotation_sin * revolution_sin) + v1 * z_rotation_cos,
                    v0 * y_rotation_cos * revolution_sin + CROSS_SECTION_RADIUS * y_rotation_sin * solid_sin,
                );

                let projection_multiplier = PROJECTION_DISTANCE / (z + OBJECT_DISTANCE);

                vertices[vertex_i].position.x = X_ORIGIN + x * projection_multiplier;
                vertices[vertex_i].position.y = Y_ORIGIN - y * projection_multiplier;

                let luminance = revolution_cos * solid_cos * z_rotation_sin - y_rotation_cos * solid_cos * revolution_sin - y_rotation_sin * solid_sin + z_rotation_cos * (y_rotation_cos * solid_sin - solid_cos * y_rotation_sin * revolution_sin);

                vertices[vertex_i].color = if luminance > 0. { Color::WHITE } else { Color::BLACK };

                vertex_i += 1;
                revolution_angle += 0.1;
            }

            solid_angle += 0.1;
        }

        window.draw_primitives(
            &vertices[..vertex_i],
            PrimitiveType::LINES,
            &RenderStates::DEFAULT,
        );
        window.display();

        y_rotation_angle += 0.03;
        z_rotation_angle += 0.03;
    }
}
