#[allow(unused_imports)]
use super::*;

impl<T: T_JsData_, E: T_JsData_> T_JsData_ for Result<T, E> {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err>
    where
        Self: Sized,
    {
        match self {
            Ok(t) => {
                writer.write_u8(0).unwrap();
                T::write_to_js(t, writer, _transfers)
            }
            Err(e) => {
                writer.write_u8(1).unwrap();
                E::write_to_js(e, writer, _transfers)
            }
        }
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        let s = reader.read_u8().map_err(|_| L_JsData_Err::Unknown)?;
        let t = match s {
            0 => Ok(T::read_from_js(reader, _transfers)?),
            1 => Err(E::read_from_js(reader, _transfers)?),
            _ => Err(L_JsData_Err::Unknown)?,
        };
        Ok(t)
    }
}
