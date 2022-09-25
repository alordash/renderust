use crate::{
    drawin::drawable::Drawable,
    geometry::primitives::{
        discrete_line::DiscreteLine, discrete_point::DiscretePoint,
        discrete_polygon::DiscretePolygon,
    },
};

pub type DiscreteTriangle = DiscretePolygon<3>;
