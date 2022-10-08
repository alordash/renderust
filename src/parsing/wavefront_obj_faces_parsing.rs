use crate::math::vector::common_vectors::vec3ui::Vec3ui;

pub fn str_parse_wavefront_faces(s: &str) -> Result<Vec<Vec3ui>, String> {
    let mut vec3is = Vec::new();
    for i in 0..3 {
        let mut vec3i = Vec3ui::new([usize::default(); 3]);
        let mut int_strings = s.split(' ');
        for j in 0..3 {
            let int_string = int_strings.next().ok_or("Not enough items")?;
            let single_int_string = int_string
                .split('/')
                .skip(i)
                .next()
                .ok_or("No backslash splitters")?;
            let int = single_int_string
                .parse::<usize>()
                .map_err(|_| "Error parsing value of ui")?;
            vec3i[j] = int;
        }
        vec3is.push(vec3i)
    }

    Ok(vec3is)
}
