use std::time::Instant;

use glam::{Mat4, Quat, Vec2, Vec3, Vec3A};
use minifb::{Key, KeyRepeat, ScaleMode, Window, WindowOptions};

use crate::{
    math::{
        geometry::apply_transform_matrix::vector_apply_transform_matrix,
        rotation::create_rotation_matrix,
    },
    plane_buffer::plane_buffer::{PlaneBuffer, PlaneBufferCreateOption},
    visual::{
        color::color::Color,
        drawing_buffer::DrawingBuffer,
        rendering::{
            ambient_occlusion::render_ambient_occlusion,
            light_source::{LightSource, LightSourceKind},
            matrix::{view_matrix::create_view_matrix, viewport_matrix::create_view_port_matrix},
            wavefront_obj::{
                wavefront_obj_depth::render_wavefront_depth,
                wavefront_obj_rendering::render_wavefront_mesh,
                wavefront_render_model::WavefrontRenderModel,
            },
        },
    },
};

use super::{
    render_config::render_config::{
        AmbientOcclusionConfig, CameraConfig, LookConfig, RenderConfigBuilder, TransformMatrixes,
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
            pitch: 0.0,
            yaw: 0.0,
            distance: 5.0,
        })
        .lights(vec![
            LightSource::new(
                LightSourceKind::Linear {
                    dir: Vec3A::Y,
                    shadow_buffer: None,
                    transform_matrix: None,
                },
                Vec3A::ONE * 0.4,
                1.0,
            ),
            LightSource::new(
                LightSourceKind::Linear {
                    dir: Vec3A::Y,
                    shadow_buffer: None,
                    transform_matrix: None,
                },
                Vec3A::ONE * 0.4,
                1.0,
            ),
            LightSource::new(LightSourceKind::Ambient, Vec3A::ONE * 0.05, 1.0),
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
    let mut rotation_matrix = create_rotation_matrix(0.0, 0.0);

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
            &mut rotation_matrix,
            t_delta,
        );

        match &mut render_config.lights[0].kind {
            LightSourceKind::Linear { dir, .. } => {
                *dir = Vec3A::new(light_spin_t.sin(), 0.0, light_spin_t.cos()).normalize();
            }
            _ => (),
        }

        draw_buffer.get_z_buffer_mut().clean_with(&f32::MIN);
        draw_buffer.clean();

        for model in render_config.models.iter() {
            let TransformMatrixes {
                view_matrix,
                projection,
                viewport_matrix,
            } = render_config.transform_matrixes;

            let mut lights = render_config.lights.clone();
            for light in lights.iter_mut() {
                match &mut light.kind {
                    LightSourceKind::Linear {
                        dir,
                        shadow_buffer: local_z_buffer,
                        transform_matrix,
                    } => {
                        if local_z_buffer.is_none() {
                            *local_z_buffer = Some(PlaneBuffer::<f32>::new(
                                draw_buffer.get_z_buffer().get_width(),
                                draw_buffer.get_z_buffer().get_height(),
                                PlaneBufferCreateOption::Fill(|_| f32::MIN),
                            ));
                        }
                        let light_rotation_matrix =
                            Mat4::from_quat(Quat::from_rotation_arc((*dir).into(), Vec3::Z));
                        // println!("l: {}", light_dir);
                        let z_buffer = local_z_buffer.as_mut().unwrap();
                        render_wavefront_depth(
                            &model,
                            z_buffer,
                            viewport_matrix,
                            projection,
                            light_rotation_matrix,
                            view_matrix,
                        );

                        let light_matrix = viewport_matrix
                            * projection
                            * model.model_matrix
                            * light_rotation_matrix
                            * view_matrix;

                        let cam_matrix = viewport_matrix
                            * projection
                            * model.model_matrix
                            * rotation_matrix
                            * view_matrix;

                        let cam_to_matrix = light_matrix * (cam_matrix.inverse());
                        *transform_matrix = Some(cam_to_matrix);
                    }
                    _ => (),
                }
            }

            render_wavefront_mesh(
                &model,
                &mut draw_buffer,
                lights,
                render_config.transform_matrixes.viewport_matrix,
                render_config.transform_matrixes.projection,
                render_config.transform_matrixes.view_matrix,
                rotation_matrix,
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

        if window.is_key_pressed(Key::X, KeyRepeat::No) {
            match &mut render_config.lights[0].kind {
                LightSourceKind::Linear {
                    shadow_buffer: local_z_buffer,
                    ..
                } => {
                    if let Some(ref mut z_buffer) = local_z_buffer {
                        z_buffer.clean_with(&f32::MIN);
                    }
                }
                _ => (),
            }
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
            render_config.camera.pitch
        ));

        if spin_light {
            light_spin_t += t_delta;
        }
    }
}
