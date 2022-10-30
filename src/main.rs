#![feature(min_specialization)]

use glam::{Mat4, Vec3};
use math::spherical_coordinate_system::spherical_to_cartesian_yzx;

pub mod math;
pub mod parsing;
pub mod plane_buffer;
pub mod visual;
pub mod wavefront;

use minifb::{Key, ScaleMode, Window, WindowOptions};
use plane_buffer::plane_buffer::PlaneBufferCreateOption;
use std::time::Instant;
use visual::{
    drawing_buffer::DrawingBuffer,
    rendering::{
        view_matrix::create_view_matrix, viewport_matrix::create_view_port_matrix,
        wavefront_obj::wavefront_obj_rendering::render_wavefront_mesh,
    },
};
use wavefront::{wavefront_obj::WavefrontObj, wavefront_obj_sources::WaveFrontObjSource};

const BUFFER_WIDTH: usize = 1000;
const BUFFER_HEIGHT: usize = 1000;

const WINDOW_WIDTH: usize = 1000;
const WINDOW_HEIGHT: usize = 1000;

const AFRO_MODEL: WaveFrontObjSource = WaveFrontObjSource::new(
    "./resources/african_head.obj",
    "./resources/african_head_diffuse.tga",
    "./resources/african_head_nm_tangent.tga",
);

const FLOOR_MODEL: WaveFrontObjSource = WaveFrontObjSource::new(
    "./resources/floor.obj",
    "./resources/floor_diffuse.tga",
    "./resources/floor_nm_tangent.tga",
);

const DIABLO_MODEL: WaveFrontObjSource = WaveFrontObjSource::new(
    "./resources/diablo3_pose.obj",
    "./resources/diablo3_pose_diffuse.tga",
    "./resources/diablo3_pose_nm_tangent.tga",
);

fn main() -> Result<(), String> {
    // Allocate the output buffer.
    let mut draw_buffer =
        DrawingBuffer::new(BUFFER_WIDTH, BUFFER_HEIGHT, PlaneBufferCreateOption::Blank);

    let mut window = Window::new(
        "Press ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    let mut light_dir = Vec3::new(0.0, 0.0, 1.0).normalize();
    let look_dir = Vec3::new(0.0, 0.0, 1.0).normalize();

    let afro_obj = WavefrontObj::from_sources_struct(&AFRO_MODEL)?;
    let floor_obj = WavefrontObj::from_sources_struct(&FLOOR_MODEL)?;
    let diablo_obj = WavefrontObj::from_sources_struct(&DIABLO_MODEL)?;

    let mut from = Vec3::new(0.0, 0.0, 10.0);
    let to = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::Y;

    let mut spin_light = true;
    let mut light_spin_t = 0.0f32;

    let mut cam_angle_theta = 0.5;
    let mut cam_angle_phi = 0.0;
    let mut cam_distance = 5.0;

    let mut view_matrix = create_view_matrix(from, to, up);
    let (w_f32, h_f32) = (
        draw_buffer.get_width() as f32,
        draw_buffer.get_height() as f32,
    );
    let viewport_matrix = create_view_port_matrix(
        w_f32 * 0.125,
        h_f32 * 0.125,
        w_f32 / 1.25,
        h_f32 / 1.25,
        255.0,
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = Instant::now();

        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            spin_light = !spin_light;
        }

        light_dir = Vec3::new(light_spin_t.sin(), 0.0, light_spin_t.cos()).normalize();

        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
            cam_angle_theta = (y / WINDOW_HEIGHT as f32) * std::f32::consts::PI;
            cam_angle_phi = ((x - WINDOW_WIDTH as f32 / 2.0) / WINDOW_WIDTH as f32)
                * std::f32::consts::PI
                * 2.0;

            from = spherical_to_cartesian_yzx(cam_angle_theta, cam_angle_phi, cam_distance).into();

            view_matrix = create_view_matrix(from, to, up);
        }

        let mut projection = Mat4::IDENTITY;
        projection.col_mut(2)[3] = -1.0 / from.distance(to);

        draw_buffer.get_z_buffer_mut().clean_with(&i32::MIN);
        draw_buffer.clean();

        render_wavefront_mesh(
            &diablo_obj,
            &mut draw_buffer,
            light_dir,
            look_dir,
            None,
            window.is_key_down(Key::LeftShift),
            projection,
            view_matrix,
            viewport_matrix,
        );

        if let Some((_, scroll_y)) = window.get_scroll_wheel() {
            let diff = -scroll_y / 100.0;
            cam_distance = (cam_distance + diff).max(0.85);
        }

        window
            .update_with_buffer(
                draw_buffer.get_buffer_as_u32_ref(),
                draw_buffer.get_width(),
                draw_buffer.get_height(),
            )
            .unwrap();

        let end = Instant::now();
        let elapsed = (end - start).as_secs_f32();

        window.set_title(&format!(
            "{:.1?} FPS, θ: {}, φ: {}, r: {}",
            1.0 / elapsed,
            cam_angle_theta,
            cam_angle_phi,
            from.distance(to)
        ));

        if spin_light {
            light_spin_t += elapsed;
        }
    }

    Ok(())
}
