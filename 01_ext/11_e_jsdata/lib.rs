#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use e_include::*;

mod err_frame;
mod err_stack;
mod hvec;
mod l_jsdata_impl;
mod webrtc;
mod xos_js;
mod xos_raw_msg;
pub use err_frame::*;
pub use err_stack::*;
pub use hvec::*;
pub use l_jsdata_impl::*;
pub use webrtc::*;
pub use xos_js::*;
pub use xos_raw_msg::*;

mod big_enum;
pub use big_enum::*;
use e_include::ordered_float::OrderedFloat;

impl<T: T_JsData_ + ordered_float::FloatCore> T_JsData_ for OrderedFloat<T> {
    fn write_to_js(&self, writer: T_JsData_Write, transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        T::write_to_js(&self.0, writer, transfers)
    }

    fn read_from_js(reader: T_JsData_Read, transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        Ok(OrderedFloat::from(T::read_from_js(reader, transfers)?))
    }
}
