#[allow(unused_imports)]
use super::*;

/*
pub struct YYY_MM_Enum_Slice {}

#[macro_export]
macro_rules! mm_enum_slice {
    (
        $( #[$meta:meta] )*
        $vis:vis enum $name:ident {
            $(
            $(#[$meta2:meta])*

            $var:ident

            ),* $(,)?
        }
    ) => {
        $( #[$meta] )*
        $vis enum $name {
            $(
            $(#[$meta2])*
            $var,)*
        }
        impl $name {
            pub const ALL: &'static [Self] = {
                use $name::*;
                &[ $($var),* ]
            };
            pub const fn all_enums() -> &'static [Self] {
                use $name::*;
                &[ $($var),* ]
            }
        }
    };
}


 */
