use std::time::Instant;

use glam::{Vec2, Vec3A};
use minifb::{Key, KeyRepeat, ScaleMode, Window, WindowOptions};

use crate::{
    plane_buffer::plane_buffer::PlaneBufferCreateOption,
    visual::{
        drawing_buffer::DrawingBuffer,
        rendering::{
            ambient_occlusion::render_ambient_occlusion,
            light_source::{LightSource, LightSourceKind},
            matrix::viewport_matrix::create_view_port_matrix,
            wavefront_obj::{
                wavefront_obj_rendering::render_wavefront_mesh,
                wavefront_render_model::WavefrontRenderModel,
            },
        },
    },
};

use super::{
    render_config::render_config::{
        AmbientOcclusionConfig, CameraConfig, LookConfig, RenderConfigBuilder,
    },
    scene_control::{
        render_config_control::handle_render_config_controls,
        scene_camera_control::handle_camera_controls,
    },
};

pub fn open_render_window(
    buffer_width: usize,
    buffer_height: usize,
    window_width: usize,
    window_height: usize,
    z_buffer_size: f32,
    models: Vec<WavefrontRenderModel>,
) {
    let mut draw_buffer =
        DrawingBuffer::new(buffer_width, buffer_height, PlaneBufferCreateOption::Blank);

    let (w_f32, h_f32) = (
        draw_buffer.get_width() as f32,
        draw_buffer.get_height() as f32,
    );

    let mut render_config = RenderConfigBuilder::default()
        .look(LookConfig {
            from: 5.0 * Vec3A::Z,
            to: Vec3A::ZERO,
            up: Vec3A::Y,
        })
        .camera(CameraConfig {
            pitch: std::f32::consts::FRAC_PI_2,
            yaw: 0.0,
            distance: 5.0,
        })
        .lights(vec![
            LightSource::new(LightSourceKind::Linear(Vec3A::Z), Vec3A::ONE * 0.85, 1.0),
            LightSource::new(LightSourceKind::Ambient, Vec3A::ONE * 0.25, 1.0),
        ])
        .ambient_occlusion(AmbientOcclusionConfig {
            apply: false,
            effect_radius: 10.0,
            intensity: 0.5,
        })
        .transform_matrixes(create_view_port_matrix(
            w_f32 * 0.125,
            h_f32 * 0.125,
            w_f32 / 1.25,
            h_f32 / 1.25,
            z_buffer_size,
        ))
        .models(models)
        .build()
        .unwrap();

    let mut window = Window::new(
        "Renderust",
        window_width,
        window_height,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    let mut spin_light = false;
    let mut mouse_pressed = false;
    let mut mouse_down_pos = Vec2::ZERO;

    let mut light_spin_t = 0.0f32;
    let mut t_delta = 0.0;

    while window.is_open() {
        let start = Instant::now();

        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            spin_light = !spin_light;
        }

        handle_render_config_controls(&window, &mut render_config);
        handle_camera_controls(
            &window,
            &mut render_config,
            &mut mouse_down_pos,
            &mut mouse_pressed,
            t_delta,
        );

        render_config.lights[0].kind = LightSourceKind::Linear(
            Vec3A::new(light_spin_t.sin(), 1.0, light_spin_t.cos()).normalize(),
        );

        draw_buffer.get_z_buffer_mut().clean_with(&f32::MIN);
        draw_buffer.clean();

        for model in render_config.models.iter() {
            render_wavefront_mesh(
                &model,
                &mut draw_buffer,
                render_config.lights.clone(),
                render_config.transform_matrixes.viewport_matrix,
                render_config.transform_matrixes.projection,
                render_config.transform_matrixes.view_matrix,
            );
        }

        if render_config.ambient_occlusion.apply {
            render_ambient_occlusion(
                &mut draw_buffer,
                z_buffer_size,
                render_config.ambient_occlusion.effect_radius,
                render_config.ambient_occlusion.intensity,
            );
        }

        window
            .update_with_buffer(
                draw_buffer.get_buffer_as_u32_ref(),
                draw_buffer.get_width(),
                draw_buffer.get_height(),
            )
            .unwrap();

        let end = Instant::now();

        t_delta = (end - start).as_secs_f32();

        window.set_title(&format!(
            "Renderust {:1.1?} FPS, [R] light {}, yaw: {:1.2}, pitch: {:1.2}",
            1.0 / t_delta,
            if spin_light { "spinning" } else { "fixed" },
            render_config.camera.yaw,
            render_config.camera.pitch,
        ));

        if spin_light {
            light_spin_t += t_delta;
        }
    }
}
