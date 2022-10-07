use std::ops::Range;

use crate::{
    drawin::color::Color,
    geometry::{
        math_vectors::vec3f::Vec3f,
        primitives::{line::Line, point::Point},
    },
};

pub struct LineXAxisCalculator {
    begin: Point,
    end: Point,
    dx: isize,
    dy: isize,
    dz: isize,
    duv: Vec3f,
    dnormal: Vec3f,
}

impl LineXAxisCalculator {
    pub fn new(mut begin: Point, mut end: Point) -> LineXAxisCalculator {
        if end.x() < begin.x() {
            (begin, end) = (end, begin);
        }
        let dx = end.x() - begin.x();
        let dy = end.y() - begin.y();
        let dz = end.z() - begin.z();
        let duv = match (begin.uv, end.uv) {
            (Some(buv), Some(euv)) => euv - buv,
            _ => Default::default(),
        };
        let dnormal = match (begin.normal, end.normal) {
            (Some(bnm), Some(enm)) => enm - bnm,
            _ => Default::default(),
        };
        LineXAxisCalculator {
            begin,
            end,
            dx,
            dy,
            dz,
            duv,
            dnormal,
        }
    }

    pub fn calculate_y_value(&self, x: isize) -> isize {
        // attempt to divide by zero
        (x - self.begin.x()) * self.dy / self.dx + self.begin.y()
    }

    pub fn calculate_y_and_z_and_uv_and_normal_value(
        &self,
        x: isize,
    ) -> (isize, isize, Vec3f, Vec3f) {
        // attempt to divide by zero
        let d = x - self.begin.x();
        let inv_dx = 1.0 / self.dx as f32;
        (
            d * self.dy / self.dx + self.begin.y(),
            d * self.dz / self.dx + self.begin.z(),
            self.duv * d as f32 * inv_dx + unsafe { self.begin.uv.unwrap_unchecked() },
            self.dnormal * d as f32 * inv_dx + unsafe { self.begin.normal.unwrap_unchecked() },
        )
    }

    pub fn calculate_y_and_z_value(&self, x: isize) -> (isize, isize) {
        // attempt to divide by zero
        let d = x - self.begin.x();
        (
            d * self.dy / self.dx + self.begin.y(),
            d * self.dz / self.dx + self.begin.z(),
        )
    }

    pub fn calculate_y_and_z_and_color_value(&self, x: isize) -> (isize, isize, Color) {
        // attempt to divide by zero
        let d = x - self.begin.x();
        let color = self
            .begin
            .color
            .interpolate(self.end.color, d as i32, self.dx as i32);
        (
            d * self.dy / self.dx + self.begin.y(),
            d * self.dz / self.dx + self.begin.z(),
            color,
        )
    }

    pub fn calculate_y_and_color_value(&self, x: isize) -> (isize, Color) {
        // attempt to divide by zero
        let d = x - self.begin.x();
        let color = self
            .begin
            .color
            .interpolate(self.end.color, d as i32, self.dx as i32);
        (d * self.dy / self.dx + self.begin.y(), color)
    }

    pub fn get_x_calculation_range(&self) -> Range<isize> {
        self.begin.x()..self.end.x()
    }
}

impl From<Line> for LineXAxisCalculator {
    fn from(line: Line) -> Self {
        LineXAxisCalculator::new(line.begin, line.end)
    }
}
