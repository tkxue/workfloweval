#[allow(unused_imports)]
use super::*;

impl<T: T_JsData_> T_JsData_ for Rc<T> {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        self.as_ref().write_to_js(writer, _transfers)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<Rc<T>, L_JsData_Err> {
        Ok(Rc::new(T::read_from_js(reader, _transfers)?))
    }
}
