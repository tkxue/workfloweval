#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use c_egui_base::{C_LogEntry, Cmsg_HGfx, Cmsg_LogViewer};
use e_api::ArcState;
use std::sync::OnceLock;

mod circular_vec;
pub use circular_vec::*;

mod c_state_log_viewer;
mod c_state_repl;
pub use c_state_log_viewer::*;
pub use c_state_repl::*;
