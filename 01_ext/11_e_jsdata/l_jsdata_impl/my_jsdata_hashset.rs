#[allow(unused_imports)]
use super::*;

impl<K: T_JsData_ + Hash + Eq> T_JsData_ for HashSet<K> {
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
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<HashSet<K>, L_JsData_Err> {
        let len = <u64 as T_JsData_>::read_from_js(reader, _transfers)? as usize;
        let mut ans = HashSet::new();
        for _i in 0..len {
            let k = <K as T_JsData_>::read_from_js(reader, _transfers)?;
            ans.insert(k);
        }
        Ok(ans)
    }
}
