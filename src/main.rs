#![allow(non_snake_case)]

#[macro_use]
extern crate derive_builder;

pub mod math;
pub mod parsing;
pub mod plane_buffer;
pub mod ui;
pub mod visual;
pub mod wavefront;

use ui::render_window::render_window::open_render_window;
use wavefront::{wavefront_obj::WavefrontObj, wavefront_obj_source::WaveFrontObjSource};

const BUFFER_WIDTH: usize = 1000;
const BUFFER_HEIGHT: usize = 1000;

const WINDOW_WIDTH: usize = 1000;
const WINDOW_HEIGHT: usize = 1000;

const Z_BUFFER_SIZE: f32 = 255.0;

const AFRO_MODEL: WaveFrontObjSource = WaveFrontObjSource::new(
    "./resources/african_head.obj",
    "./resources/african_head_diffuse.tga",
    Some("./resources/african_head_nm_tangent.tga"),
    None,
    None
);

const FLOOR_MODEL: WaveFrontObjSource = WaveFrontObjSource::new(
    "./resources/floor.obj",
    "./resources/floor_diffuse.tga",
    Some("./resources/floor_nm_tangent.tga"),
    None,
    None
);

const DIABLO_MODEL: WaveFrontObjSource = WaveFrontObjSource::new(
    "./resources/diablo3_pose.obj",
    "./resources/diablo3_pose_diffuse.tga",
    Some("./resources/diablo3_pose_nm_tangent.tga"),
    Some("./resources/diablo3_pose_spec.tga"),
    Some("./resources/diablo3_pose_glow.tga")
);

fn main() -> Result<(), String> {
    let afro_obj = WavefrontObj::from_sources_struct(&AFRO_MODEL)?;
    let floor_obj = WavefrontObj::from_sources_struct(&FLOOR_MODEL)?;
    let diablo_obj = WavefrontObj::from_sources_struct(&DIABLO_MODEL)?;

    open_render_window(
        BUFFER_WIDTH,
        BUFFER_HEIGHT,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        Z_BUFFER_SIZE,
        vec![diablo_obj.into()],
    );

    Ok(())
}
