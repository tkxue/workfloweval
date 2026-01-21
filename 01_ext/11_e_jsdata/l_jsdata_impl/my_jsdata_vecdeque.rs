use super::*;

impl<T: T_JsData_> T_JsData_ for VecDeque<T> {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        let len = self.len() as u64;
        len.write_to_js(writer, _transfers)?;
        for x in self.iter() {
            x.write_to_js(writer, _transfers)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<VecDeque<T>, L_JsData_Err> {
        let len = <u64 as T_JsData_>::read_from_js(reader, _transfers)? as usize;
        let mut ans = VecDeque::new();
        for _i in 0..len {
            ans.push_back(<T as T_JsData_>::read_from_js(reader, _transfers)?);
        }
        Ok(ans)
    }
}
