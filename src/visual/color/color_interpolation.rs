use super::color::Color;

impl Color {
    pub fn interpolate(self, rhs: Color, t: i32, t_max: i32) -> Color {
        let r = (((rhs.r as i32 - self.r as i32) * t / t_max) as u8).wrapping_add(self.r);
        let g = (((rhs.g as i32 - self.g as i32) * t / t_max) as u8).wrapping_add(self.g);
        let b = (((rhs.b as i32 - self.b as i32) * t / t_max) as u8).wrapping_add(self.b);
        Color { r, g, b, ..self }
    }

    pub fn interpolate_multiple(colors: &Vec<Color>, ts: Vec<f32>, t_total: f32) -> Color {
        let r = colors
            .iter()
            .zip(ts.iter())
            .map(|(c, t)| (c.r as f32) * t / (t_total))
            .sum::<f32>()
            .max(0.0) as u8;
        let g = colors
            .iter()
            .zip(ts.iter())
            .map(|(c, t)| (c.g as f32) * t / (t_total))
            .sum::<f32>()
            .max(0.0) as u8;
        let b = colors
            .iter()
            .zip(ts.iter())
            .map(|(c, t)| (c.b as f32) * t / (t_total))
            .sum::<f32>()
            .max(0.0) as u8;
        Color {
            r,
            g,
            b,
            ..Default::default()
        }
    }
}
