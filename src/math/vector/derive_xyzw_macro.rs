#[macro_export]
macro_rules! impl_xyzw_fns {
    ($val_ty:ty, 1) => {
        pub fn x(&self) -> $val_ty {
            self.0[0]
        }
        pub fn x_mut(&mut self) -> &mut $val_ty {
            &mut self.0[0]
        }
        pub fn set_x(&mut self, rhs: $val_ty) {
            self.0[0] = rhs;
        }
    };
    ($val_ty:ty, 2) => {
        pub fn y(&self) -> $val_ty {
            self.0[1]
        }
        pub fn y_mut(&mut self) -> &mut $val_ty {
            &mut self.0[1]
        }
        pub fn set_y(&mut self, rhs: $val_ty) {
            self.0[1] = rhs;
        }
        crate::impl_xyzw_fns!($val_ty, 1);
    };
    ($val_ty:ty, 3) => {
        pub fn z(&self) -> $val_ty {
            self.0[2]
        }
        pub fn z_mut(&mut self) -> &mut $val_ty {
            &mut self.0[2]
        }
        pub fn set_z(&mut self, rhs: $val_ty) {
            self.0[2] = rhs;
        }
        crate::impl_xyzw_fns!($val_ty, 2);
    };
    ($val_ty:ty, 4) => {
        pub fn w(&self) -> $val_ty {
            self.0[3]
        }
        pub fn w_mut(&mut self) -> &mut $val_ty {
            &mut self.0[3]
        }
        pub fn set_w(&mut self, rhs: $val_ty) {
            self.0[3] = rhs;
        }
        crate::impl_xyzw_fns!($val_ty, 3);
    };
}

#[macro_export]
macro_rules! derive_xyzw {
    ($t:ident <
        $size:tt,
        $( $gen_type:tt ),+
    >, $val_ty:ty) => {
        impl < $( $gen_type ),+ > $t <$size, $( $gen_type ),+ > where $val_ty : Copy {
            crate::impl_xyzw_fns!($val_ty, $size);
        }
    };
}
