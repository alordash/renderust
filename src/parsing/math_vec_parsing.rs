use std::{any::type_name, str::FromStr};

use crate::math::vector::math_vec::MathVec;

pub fn str_parse_math_vec<const N: usize, T: Default + FromStr + Copy>(
    string: &str,
) -> Result<MathVec<N, T>, String> {
    let mut words = string.split_whitespace();
    let mut math_vec = MathVec::new([T::default(); N]);
    for i in 0..N {
        let value_string = words.next().ok_or("Not enough items")?;
        let value = value_string
            .parse::<T>()
            .map_err(|_| format!("Error parsing value of \"{}\"", type_name::<T>()))?;
        math_vec[i] = value;
    }
    Ok(math_vec)
}
