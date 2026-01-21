use super::*;
// use e_include::chacha20::cipher::consts::U16;

mod my_jsdata_arc;
mod my_jsdata_array;
mod my_jsdata_box;
mod my_jsdata_btreemap;
mod my_jsdata_hashmap;
mod my_jsdata_hashset;
mod my_jsdata_inner_dyn;
mod my_jsdata_option;
mod my_jsdata_prim;
mod my_jsdata_rc;
mod my_jsdata_result;
mod my_jsdata_rov;
mod my_jsdata_string;
mod my_jsdata_tup;
mod my_jsdata_vec;
mod my_jsdata_vecdeque;

mod u16_big_enum;
pub use u16_big_enum::*;

pub enum Rov<'a, T> {
    Ref(&'a T),
    Value(T),
}

pub use my_jsdata_inner_dyn::*;

pub type T_JsData_Write<'a, 'b> = &'a mut BufWriter<&'b mut Vec<u8>>;
pub type T_JsData_Read<'a, 'b> = &'a mut BufReader<&'b [u8]>;

pub trait T_JsData_ {
    fn write_to_js(&self, writer: T_JsData_Write, transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err>;

    fn read_from_js(reader: T_JsData_Read, transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized;
}

pub struct JsData_Util {}

impl JsData_Util {
    fn js_write_u8_slice(
        writer: T_JsData_Write,
        obj: &[u8],
        _transfers: &mut VecDeque<wb::JsValue>,
    ) -> Result<(), L_JsData_Err> {
        let len = obj.len() as u64;
        len.write_to_js(writer, _transfers)?;
        Ok(writer.write_all(obj).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    fn js_read_u8_slice(
        reader: T_JsData_Read,
        _transfers: &mut VecDeque<wb::JsValue>,
    ) -> Result<Vec<u8>, L_JsData_Err> {
        let n: u64 = T_JsData_::read_from_js(reader, _transfers)?;
        let mut ans = vec![0; n as usize];
        reader.read_exact(&mut ans).map_err(|_| L_JsData_Err::BufReader)?;
        Ok(ans)
    }
}

pub struct L_JsData_Util {}

impl L_JsData_Util {
    pub fn obj_to_bytes<T: T_JsData_>(obj: &T) -> Vec<u8> {
        let mut out = vec![];
        let mut bw = BufWriter::new(&mut out);
        let mut t2 = VecDeque::new();
        let _ = <T as T_JsData_>::write_to_js(&obj, &mut bw, &mut t2);
        drop(bw);
        out
    }

    pub fn read_obj<T: T_JsData_>(x: &[u8]) -> Result<T, L_JsData_Err> {
        let mut bf = BufReader::new(x);
        let mut t2 = VecDeque::new();
        <T as T_JsData_>::read_from_js(&mut bf, &mut t2)
    }
}
