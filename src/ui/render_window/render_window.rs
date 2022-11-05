use std::{cell::RefCell, time::Instant};

use glam::{Mat4, Vec3A};
use minifb::{Key, ScaleMode, Window, WindowOptions};

use crate::{
    math::spherical_coordinate_system::spherical_to_cartesian_yzx,
    plane_buffer::plane_buffer::PlaneBufferCreateOption,
    ui::input_binding::{InputBinding, InputBindingKind},
    visual::{
        drawing_buffer::DrawingBuffer,
        rendering::{
            ambient_occlusion::render_ambient_occlusion,
            light_source::{LightSource, LightSourceKind},
            matrix::{view_matrix::create_view_matrix, viewport_matrix::create_view_port_matrix},
            wavefront_obj::{
                wavefront_obj_rendering::render_wavefront_mesh,
                wavefront_render_model::WavefrontRenderModel,
            },
        },
    },
};

use super::render_config::render_config::{
    AmbientOcclusionConfig, CameraConfig, LookConfig, RenderConfigBuilder,
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

    let render_config = RefCell::new(
        RenderConfigBuilder::default()
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
                LightSource::new(LightSourceKind::Linear(Vec3A::Z), Vec3A::ONE * 10.0, 10.0),
                LightSource::new(LightSourceKind::Ambient, Vec3A::ONE * 0.33, 1.0),
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
            .unwrap(),
    );

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

    let spin_light = RefCell::new(false);

    let mut light_spin_t = 0.0f32;
    let mut t_delta = 0.0;

    let mut input_bindings = [
        InputBinding::new(
            Key::Space,
            InputBindingKind::KeyPressed(minifb::KeyRepeat::No),
            || {
                let mut spin_light = RefCell::borrow_mut(&spin_light);
                *spin_light = !*spin_light;
            },
        ),
        InputBinding::new(
            Key::Key1,
            InputBindingKind::KeyPressed(minifb::KeyRepeat::No),
            || {
                let mut render_config = RefCell::borrow_mut(&render_config);
                for model in render_config.models.iter_mut() {
                    model.use_normal_map = !model.use_normal_map;
                }
            },
        ),
        InputBinding::new(
            Key::Key2,
            InputBindingKind::KeyPressed(minifb::KeyRepeat::No),
            || {
                let mut render_config = RefCell::borrow_mut(&render_config);
                render_config.ambient_occlusion.apply = !render_config.ambient_occlusion.apply;
            },
        ),
    ];

    while window.is_open() {
        let start = Instant::now();

        InputBinding::handle_inputs(&window, &mut input_bindings);

        let mut render_config = RefCell::borrow_mut(&render_config);

        let spin_light = RefCell::borrow(&spin_light);

        render_config.lights[0].kind = LightSourceKind::Linear(
            Vec3A::new(light_spin_t.sin(), 1.0, light_spin_t.cos()).normalize(),
        );

        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
            let (yaw, pitch, distance) = {
                let CameraConfig {
                    yaw,
                    pitch,
                    distance,
                } = &mut (render_config.camera);
                *yaw = (y / window_height as f32) * std::f32::consts::PI;
                *pitch = ((x - window_width as f32 / 2.0) / buffer_width as f32)
                    * std::f32::consts::PI
                    * 2.0;
                (*yaw, *pitch, *distance)
            };
            {
                let LookConfig { from, to, up } = &mut (render_config.look);

                *from = spherical_to_cartesian_yzx(yaw, pitch, distance).into();

                render_config.transform_matrixes.view_matrix = create_view_matrix(*from, *to, *up);
            }
        }

        render_config.transform_matrixes.projection = Mat4::IDENTITY;
        render_config.transform_matrixes.projection.col_mut(2)[3] =
            -1.0 / render_config.look.from.distance(render_config.look.to);

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
            "Renderust {:1.1?} FPS, [SPACE] light {}",
            1.0 / t_delta,
            if *spin_light { "spinning" } else { "fixed" }
        ));

        if *spin_light {
            light_spin_t += t_delta;
        }
    }
}
