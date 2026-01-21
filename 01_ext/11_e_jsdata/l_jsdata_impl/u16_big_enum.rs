use super::*;

pub struct U16_BigEnum<T: T_BigEnum_ + ?Sized> {
    pub inner: u16,
    _t: PhantomData<T>,
}

impl<T: T_BigEnum_> U16_BigEnum<T> {
    pub fn to_orig(&self) -> Option<T> {
        T::from_u16_(self.inner)
    }

    pub fn new(x: u16) -> U16_BigEnum<T> {
        U16_BigEnum {
            inner: x,
            _t: Default::default(),
        }
    }
}

impl<T: T_BigEnum_> T_JsData_ for U16_BigEnum<T> {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        let _ = writer.write_u16::<LittleEndian>(self.inner);
        Ok(())
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        let t = reader.read_u16::<LittleEndian>().map_err(|_e| L_JsData_Err::BufReader)?;
        Ok(U16_BigEnum::new(t))
    }
}
