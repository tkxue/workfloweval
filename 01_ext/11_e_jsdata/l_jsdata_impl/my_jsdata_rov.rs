use super::*;

#[allow(unused_imports)]
use super::*;

impl<'a, T: T_JsData_> T_JsData_ for Rov<'a, T> {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        match self {
            Rov::Ref(x) => x.write_to_js(writer, _transfers),
            Rov::Value(x) => x.write_to_js(writer, _transfers),
        }
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<Rov<'a, T>, L_JsData_Err> {
        let t = T::read_from_js(reader, _transfers)?;
        Ok(Rov::Value(t))
    }
}
