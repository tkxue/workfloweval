#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub use anymap;
pub use arrayvec;
pub use base64;
pub use bitflags;
pub use bitvec;
pub use byteorder;
pub use bytes;
pub use flate2;
pub use gensym;
pub use hex;
pub use instant;
pub use itertools;
pub use js_sys;
pub use lazy_static;
pub use nanoserde;
pub use ordered_float;
pub use rand;
// pub use rand_pcg;
// pub use eetf;
// pub use erlang_term;
pub use regex;
pub use reqwest;
//pub use serde_eetf;
// pub use sha1;
// pub use tar;
pub use tinyvec;
// pub use url;
pub use wasm_bindgen as wb;

pub use arrayvec::ArrayVec;

mod arc_state;
mod msg_queue;
mod wasm_init;
pub use arc_state::*;
pub use msg_queue::*;
pub use wasm_init::*;

pub use std::{
    any::Any,
    borrow::{Borrow, Cow},
    cell::*,
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    fmt::{Debug, Display, Formatter},
    future::Future,
    hash::{Hash, Hasher},
    io::{BufReader, BufWriter, Error, Read, Write},
    iter::{Enumerate, Map},
    marker::PhantomData,
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8,
    },
    ops::{Add, Deref, DerefMut, Index, Mul, Rem},
    pin::Pin,
    rc::{Rc, Weak},
    str::FromStr,
    sync::atomic::*,
    sync::*,
};

pub struct Image_Util {}

impl Image_Util {
    /*
    pub fn read(data: &[u8]) -> image::DynamicImage {
        let cursor = std::io::Cursor::new(data);
        let mut img_reader = image::io::Reader::new(cursor);
        img_reader.no_limits();
        let img = img_reader.with_guessed_format().unwrap().decode().unwrap();
        img
    }

     */
}

#[derive(Debug)]
pub enum L_Serde_Err {
    BufWriter,
    Illegal_Enum,
    Option,
    StringConv,
    BufReader,
    Unknown,
    HVec_Wrong_Size,
    Other(String),
    Out_of_Space,
    JsonTag,
    Dst_Err { to: String, actual: String },
    Missing_To,
    Missing_RustPart,
    Missing_Data,
    Transfers_Missing,
    Transfers_Cant_Convert,
    Transfers_Is_Null,
}

pub type L_JsData_Err = L_Serde_Err;
pub type L_NetData_Err = L_Serde_Err;

impl std::error::Error for L_NetData_Err {}

impl std::fmt::Display for L_NetData_Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
