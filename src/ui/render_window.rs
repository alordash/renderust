use std::time::Instant;

use glam::{Mat4, Vec3A};
use minifb::{Key, ScaleMode, Window, WindowOptions};

use crate::{
    math::spherical_coordinate_system::spherical_to_cartesian_yzx,
    plane_buffer::plane_buffer::PlaneBufferCreateOption,
    visual::{
        color::color::Color,
        drawing_buffer::DrawingBuffer,
        rendering::{
            ambient_occlusion::render_ambient_occlusion,
            light_source::{LightSource, LightSourceKind},
            wavefront_obj::{
                wavefront_obj_rendering::render_wavefront_mesh,
                wavefront_render_model::WavefrontRenderModel,
            }, matrix::{viewport_matrix::create_view_port_matrix, view_matrix::create_view_matrix},
        },
    },
    wavefront::wavefront_obj::WavefrontObj,
};

use super::render_config::render_config::{
    AmbientOcclusionConfig, CameraConfig, LookConfig, RenderConfigBuilder,
};

const INTENSITIES: [Vec3A; 3] = [Vec3A::X, Vec3A::Y, Vec3A::Z];

pub fn open_render_window(
    buffer_width: usize,
    buffer_height: usize,
    window_width: usize,
    window_height: usize,
    z_buffer_size: f32,
    mut models: Vec<WavefrontRenderModel>,
) {
    let mut draw_buffer =
        DrawingBuffer::new(buffer_width, buffer_height, PlaneBufferCreateOption::Blank);

    let (w_f32, h_f32) = (
        draw_buffer.get_width() as f32,
        draw_buffer.get_height() as f32,
    );

    let mut render_config = RenderConfigBuilder::default()
        .look(LookConfig {
            from: Vec3A::Z,
            to: Vec3A::ZERO,
            up: Vec3A::Y,
        })
        .camera(CameraConfig {
            yaw: 0.0,
            pitch: 0.0,
            distance: 5.0,
        })
        .lights(vec![
            // LightSource::new(LightSourceKind::Linear(Vec3A::X), Vec3A::ONE * 0.5, 1.0),
            LightSource::new(LightSourceKind::Point(Vec3A::Z * 0.1), Vec3A::ONE * 20.0, 20.0), // LightSource::new(LightSourceKind::Linear(Vec3A::Z), INTENSITIES[2]),
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

    let mut intensity_idx = 0usize;

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

    let a = Vec3A::X.cross(Vec3A::X);

    let mut spin_light = false;
    let mut light_spin_t = 0.0f32;
    let mut t_delta = 0.0;

    while window.is_open() {
        let start = Instant::now();

        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            spin_light = !spin_light;
        }

        if window.is_key_pressed(Key::LeftCtrl, minifb::KeyRepeat::No) {
            render_config.models[0].use_normal_map = !render_config.models[0].use_normal_map;
        }

        if window.is_key_pressed(Key::A, minifb::KeyRepeat::No) {
            render_config.ambient_occlusion.apply = !render_config.ambient_occlusion.apply;
        }

        // render_config.lights[0].kind = LightSourceKind::Linear(
        //     Vec3A::new(light_spin_t.sin(), 1.0, light_spin_t.cos()).normalize(),
        // );

        render_config.lights[0].kind = LightSourceKind::Point(
            Vec3A::new(0.0, 0.0, light_spin_t.sin() * 2.0),
        );

        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
            let CameraConfig {
                yaw,
                pitch,
                distance,
            } = &mut render_config.camera;
            *yaw = (y / window_height as f32) * std::f32::consts::PI;
            *pitch = ((x - window_width as f32 / 2.0) / buffer_width as f32)
                * std::f32::consts::PI
                * 2.0;

            let LookConfig { from, to, up } = &mut render_config.look;

            *from = spherical_to_cartesian_yzx(*yaw, *pitch, *distance).into();

            render_config.transform_matrixes.view_matrix = create_view_matrix(*from, *to, *up);
        }

        let mut projection = Mat4::IDENTITY;
        projection.col_mut(2)[3] = -1.0 / render_config.look.from.distance(render_config.look.to);

        draw_buffer.get_z_buffer_mut().clean_with(&f32::MIN);
        draw_buffer.clean();

        for model in render_config.models.iter() {
            render_wavefront_mesh(
                &model,
                &mut draw_buffer,
                render_config.lights.clone(),
                render_config.transform_matrixes.viewport_matrix,
                projection,
                render_config.transform_matrixes.view_matrix,
                z_buffer_size,
            );
        }

        if render_config.ambient_occlusion.apply {
            render_ambient_occlusion(&mut draw_buffer, z_buffer_size, 10.0, 0.5);
        }

        if let Some((_, scroll_y)) = window.get_scroll_wheel() {
            let diff = -scroll_y / 100.0;
            render_config.camera.distance = (render_config.camera.distance + diff).max(0.85);
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
            "{:1.1?} FPS, [SPACE] light {}, [LCtrl] normal map: {}",
            1.0 / t_delta,
            if spin_light { "spinning" } else { "fixed" },
            render_config.models[0].use_normal_map,
        ));

        if spin_light {
            light_spin_t += t_delta;
        }
    }
}
