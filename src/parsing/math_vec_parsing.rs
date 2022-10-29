use std::any::type_name;

use glam::Vec3;

pub fn str_parse_vec3(string: &str) -> Result<Vec3, String> {
    let mut words = string.split_whitespace();
    let mut math_vec = Vec3::default();
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
                println!("Not enough items!");
            }
        }
    }
    Ok(math_vec)
}
