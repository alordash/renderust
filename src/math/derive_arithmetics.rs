#[macro_export]
macro_rules! derive_self_wrapping_add {
    ($t:ident $(<
        $( $gen_type:tt ),+
    >)*, $($field:tt),+) => {
        impl $(< $( $gen_type ),+ >)* std::ops::Add for $t $(< $( $gen_type ),+ >)* {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                $t {
                    $($field: self.$field.wrapping_add(rhs.$field),)+
                    ..self
                }
            }
        }
    }
}

#[macro_export]
macro_rules! derive_self_wrapping_sub {
    ($t:ident $(<
        $( $type:ty ),+
    >)*, $($field:tt),+) => {
        impl std::ops::Sub for $t {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                $t {
                    $($field: self.$field.wrapping_sub(rhs.$field),)+
                    ..self
                }
            }
        }
    }
}

#[macro_export]
macro_rules! derive_self_add {
    ($t:ident $(<
        $( $gen_type:tt ),+
    >)*, $($field:tt),+) => {
        impl $(< $( $gen_type ),+ >)* std::ops::Add for $t $(< $( $gen_type ),+ >)* {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                $t {
                    $($field: self.$field + rhs.$field,)+
                    ..self
                }
            }
        }
    }
}

#[macro_export]
macro_rules! derive_self_sub {
    ($t:ident $(<
        $( $type:ty ),+
    >)*, $($field:tt),+) => {
        impl std::ops::Sub for $t {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                $t {
                    $($field: self.$field - rhs.$field,)+
                    ..self
                }
            }
        }
    }
}

#[macro_export]
macro_rules! derive_mul_by {
    ($t:ident, $multiplier_type:ty, $($field:tt, $source_type:ty),+) => {
        impl std::ops::Mul<$multiplier_type> for $t {
            type Output = Self;
            fn mul(self, rhs: $multiplier_type) -> Self::Output {
                $t {
                    $($field: (self.$field * rhs as $source_type),)+
                    ..self
                }
            }
        }
    };
}

#[macro_export]
macro_rules! derive_div_by {
    ($t:ident, $divider_type:ty, $($field:tt, $source_type:ty),+) => {
        impl std::ops::Div<$divider_type> for $t {
            type Output = Self;
            fn div(self, rhs: $divider_type) -> Self::Output {
                $t {
                    $($field: (self.$field / rhs as $source_type),)+
                    ..self
                }
            }
        }
    };
}
