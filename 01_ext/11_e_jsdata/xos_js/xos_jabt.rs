use super::*;

pub struct Xos_JabT<T> {
    pub inner: Xos_Jab,
    _t: PhantomData<T>,
}

impl<T> Clone for Xos_JabT<T> {
    fn clone(&self) -> Self {
        Xos_JabT {
            inner: self.inner.clone(),
            _t: Default::default(),
        }
    }
}

impl<T> Debug for Xos_JabT<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "T_Js_ArrayBuffer")
    }
}

impl<T> T_JsData_ for Xos_JabT<T> {
    fn write_to_js(&self, _writer: T_JsData_Write, transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        transfers.push_back(self.inner.inner.clone().into());
        Ok(())
    }

    fn read_from_js(_reader: T_JsData_Read, transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        let ab = transfers.pop_front().unwrap();
        Ok(Xos_JabT {
            inner: Xos_Jab::new_jsv(&ab),
            _t: Default::default(),
        })
    }
}
