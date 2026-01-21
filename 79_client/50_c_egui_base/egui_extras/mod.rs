//! This is a crate that adds some features on top top of [`egui`](https://github.com/emilk/egui).
//!
//! This crate are for experimental features, and features that require big dependencies that does not belong in `egui`.
//!
//! ## Feature flags
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!

#![expect(clippy::manual_range_contains)]

mod datepicker;

pub mod syntax_highlighting;

#[doc(hidden)]
pub mod image;
mod layout;
pub mod loaders;
mod sizing;
mod strip;
mod table;

pub use crate::egui_extras::datepicker::DatePickerButton;

pub(crate) use crate::egui_extras::layout::StripLayout;
pub use crate::egui_extras::sizing::Size;
pub use crate::egui_extras::strip::*;
pub use crate::egui_extras::table::*;

pub use loaders::install_image_loaders;

// ---------------------------------------------------------------------------

/// Panic in debug builds, log otherwise.
macro_rules! log_or_panic {
    ($fmt: literal) => {$crate::log_or_panic!($fmt,)};
    ($fmt: literal, $($arg: tt)*) => {{
        if cfg!(debug_assertions) {
            panic!($fmt, $($arg)*);
        } else {
            log::error!($fmt, $($arg)*);
        }
    }};
}
pub(crate) use log_or_panic;
