#[allow(unused_imports)]
use super::*;

impl T_JsData_ for String {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        JsData_Util::js_write_u8_slice(writer, self.as_bytes(), _transfers)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<String, L_JsData_Err> {
        let x = JsData_Util::js_read_u8_slice(reader, _transfers)?;
        String::from_utf8(x).map_err(|_| L_JsData_Err::StringConv)
    }
}
