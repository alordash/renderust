use std::any::type_name;

use glam::Vec3;

pub fn str_parse_vec3(string: &str) -> Result<Vec3, String> {
    let mut words = string.split_whitespace();
    let mut math_vec = Vec3::default();
    for i in 0..3 {
        let value_string = words.next().ok_or("Not enough items")?;
        let value = value_string
            .parse::<f32>()
            .map_err(|_| format!("Error parsing value of \"{}\"", type_name::<f32>()))?;
        math_vec[i] = value;
    }
    Ok(math_vec)
}
