#[allow(unused_imports)]
use super::*;

impl<T: T_JsData_> T_JsData_ for Option<T> {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        match self {
            None => 0_u8.write_to_js(writer, _transfers),
            Some(v) => {
                1_u8.write_to_js(writer, _transfers)?;
                v.write_to_js(writer, _transfers)
            }
        }
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<Option<T>, L_JsData_Err> {
        let x: u8 = T_JsData_::read_from_js(reader, _transfers)?;
        match x {
            0 => return Ok(None),
            1 => return Ok(Some(<T as T_JsData_>::read_from_js(reader, _transfers)?)),
            _ => return Err(L_JsData_Err::Option),
        }
    }
}
