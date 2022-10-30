use std::any::type_name;

use glam::Vec3A;

pub fn str_parse_vec3(string: &str) -> Result<Vec3A, String> {
    let mut words = string.split_whitespace();
    let mut math_vec = Vec3A::default();
    for i in 0..3 {
        match words.next() {
            Some(value_string) => {
                let value = value_string
                    .parse::<f32>()
                    .map_err(|_| format!("Error parsing value of \"{}\"", type_name::<f32>()))?;
                math_vec[i] = value;
            }
            None => {
                math_vec[i] = 0.0;
            }
        }
    }
    Ok(math_vec)
}
