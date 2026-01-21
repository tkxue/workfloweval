#[allow(unused_imports)]
use super::*;

#[macro_export]
macro_rules! mm_ensure {
    (
        {
            $ensure_test:tt
        },
        {
            $ensure_err:tt
        }
    ) => {
        if !({ $ensure_test }) {
            return std::result::Result::Err({ $ensure_err });
        }
    };
}
