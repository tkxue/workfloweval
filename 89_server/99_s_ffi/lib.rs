#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

/*
mod fdb_util;
mod hello_world;
mod inv_index;
mod job_queue;
mod kv_dir;
mod sffi_global;
mod vec_search;
pub use fdb_util::*;
pub use hello_world::*;
pub use inv_index::*;
pub use job_queue::*;
pub use kv_dir::*;
pub use sffi_global::*;
pub use vec_search::*;

use mimalloc::MiMalloc;

 */

rustler::init!("Elixir.SFfi.Raw");

#[rustler::nif]
pub fn priv__init_once() {
    s_shared::G_S_Ffi::__init_once()
}
