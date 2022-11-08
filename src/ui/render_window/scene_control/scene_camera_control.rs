use glam::{Mat4, Vec2, Vec4};
use minifb::{Key, MouseButton, MouseMode, Window};

use crate::{
    math::rotation::create_rotation_matrix,
    ui::render_window::render_config::render_config::{CameraConfig, RenderConfig},
    visual::rendering::matrix::projection_matrix::create_projection_matrix,
};

const MOVE_SPEED: f32 = 2.0;
const ROTATION_SPEED: f32 = 2.0;

const MOVE_INPUTS: [(Key, Vec4); 6] = [
    (Key::W, Vec4::NEG_Z),
    (Key::S, Vec4::Z),
    (Key::A, Vec4::NEG_X),
    (Key::D, Vec4::X),
    (Key::LeftShift, Vec4::NEG_Y),
    (Key::Space, Vec4::Y),
];

pub fn handle_camera_controls(
    window: &Window,
    render_config: &mut RenderConfig,
    mouse_down_pos: &mut Vec2,
    mouse_pressed: &mut bool,
    rotation_matrix: &mut Mat4,
    t_delta: f32,
) {
    if let Some((_, y)) = window.get_scroll_wheel() {
        let diff = -y / 100.0;
        let new_distance = (render_config.camera.distance + diff).max(0.85);
        render_config.camera.distance = new_distance;

        render_config.transform_matrixes.projection = create_projection_matrix(new_distance);
    }

    let (x, y) = window.get_mouse_pos(MouseMode::Pass).unwrap();
    let pos = Vec2::new(x, y);

    if window.get_mouse_down(MouseButton::Left) {
        if !*mouse_pressed {
            *mouse_down_pos = pos;
        }
        *mouse_pressed = true;
    } else {
        *mouse_pressed = false;
    }

    if *mouse_pressed {
        let diff = Vec2::new(x, y) - *mouse_down_pos;
        *mouse_down_pos = pos;
        let CameraConfig { pitch, yaw, .. } = &mut (render_config.camera);
        let (width, height) = window.get_size();
        *pitch = *pitch + (ROTATION_SPEED * diff.y / height as f32) * std::f32::consts::PI;
        *yaw = *yaw + (ROTATION_SPEED * diff.x / width as f32) * std::f32::consts::PI * 2.0;

        *rotation_matrix = create_rotation_matrix(*yaw, *pitch);
    }

    if !window.is_key_down(Key::LeftCtrl) {
        for move_input in MOVE_INPUTS.iter() {
            if window.is_key_down(move_input.0) {
                for model in render_config.models.iter_mut() {
                    *model.model_matrix.col_mut(3) += move_input.1 * MOVE_SPEED * t_delta;
                }
            }
        }
    }
}
