#[macro_export]
macro_rules! derive_self_add {
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
macro_rules! derive_self_sub {
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