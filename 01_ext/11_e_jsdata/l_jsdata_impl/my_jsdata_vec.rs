#[allow(unused_imports)]
use super::*;

#[allow(dead_code)]
pub enum SVec<'a, T: T_JsData_> {
    Slice(&'a [T]),
    Vec(Vec<T>),
}

// =======================================

impl<T: T_JsData_> T_JsData_ for Vec<T> {
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
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<Vec<T>, L_JsData_Err> {
        let len = <u64 as T_JsData_>::read_from_js(reader, _transfers)? as usize;
        let mut ans = vec![]; // Vec::with_capacity(len);
        for _i in 0..len {
            ans.push(<T as T_JsData_>::read_from_js(reader, _transfers)?);
        }
        Ok(ans)
    }
}

// =======================================

impl<'a, T: T_JsData_> T_JsData_ for SVec<'a, T> {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        match self {
            SVec::Slice(x) => {
                let len = x.len() as u64;
                len.write_to_js(writer, _transfers)?;
                for x in x.iter() {
                    x.write_to_js(writer, _transfers)?;
                }
                Ok(())
            }
            SVec::Vec(x) => {
                let len = x.len() as u64;
                len.write_to_js(writer, _transfers)?;
                for x in x.iter() {
                    x.write_to_js(writer, _transfers)?;
                }
                Ok(())
            }
        }
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<SVec<'a, T>, L_JsData_Err> {
        let len = <u64 as T_JsData_>::read_from_js(reader, _transfers)? as usize;
        let mut ans = vec![]; // Vec::with_capacity(len);
        for _i in 0..len {
            ans.push(<T as T_JsData_>::read_from_js(reader, _transfers)?);
        }
        Ok(SVec::Vec(ans))
    }
}
