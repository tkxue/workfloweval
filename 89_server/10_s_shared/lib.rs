#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use mimalloc::MiMalloc;

mod sffi_global;
pub use sffi_global::*;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
