use minifb::{Key, KeyRepeat, Window};

use crate::ui::render_window::render_config::render_config::RenderConfig;

pub fn handle_render_config_controls(window: &Window, render_config: &mut RenderConfig) {
    if window.is_key_pressed(Key::Key1, KeyRepeat::No) {
        for model in render_config.models.iter_mut() {
            model.use_normal_map = !model.use_normal_map;
        }
    }

    if window.is_key_pressed(Key::Key2, KeyRepeat::No) {
        for model in render_config.models.iter_mut() {
            model.use_spec_map = !model.use_spec_map;
        }
    }

    if window.is_key_pressed(Key::Key3, KeyRepeat::No) {
        for model in render_config.models.iter_mut() {
            model.use_glow_map = !model.use_glow_map;
        }
    }

    if window.is_key_pressed(Key::Key4, KeyRepeat::No) {
        render_config.ambient_occlusion.apply = !render_config.ambient_occlusion.apply;
    }
}
