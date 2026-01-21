#[allow(unused_imports)]
use super::*;

impl<K: T_JsData_ + Hash + Eq, V: T_JsData_> T_JsData_ for HashMap<K, V> {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        let len = self.len() as u64;
        len.write_to_js(writer, _transfers)?;
        for x in self.iter() {
            x.0.write_to_js(writer, _transfers)?;
            x.1.write_to_js(writer, _transfers)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<HashMap<K, V>, L_JsData_Err> {
        let len = <u64 as T_JsData_>::read_from_js(reader, _transfers)? as usize;
        let mut ans = HashMap::new();
        for _i in 0..len {
            let k = <K as T_JsData_>::read_from_js(reader, _transfers)?;
            let v = <V as T_JsData_>::read_from_js(reader, _transfers)?;
            ans.insert(k, v);
        }
        Ok(ans)
    }
}
