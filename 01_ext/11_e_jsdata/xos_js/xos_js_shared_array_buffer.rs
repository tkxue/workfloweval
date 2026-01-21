#[allow(unused_imports)]
use super::*;

/*
#[derive(Clone)]
pub struct Xos_Js_SharedArrayBuffer {
    pub inner: js_sys::SharedArrayBuffer,
}

impl std::fmt::Debug for Xos_Js_SharedArrayBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Xos_Js_SharedArrayBuffer")
    }
}

impl Xos_Js_SharedArrayBuffer {
    pub fn new_jsv(x: &wb::JsValue) -> Xos_Js_SharedArrayBuffer {
        let inner = js_sys::SharedArrayBuffer::from(x.clone());
        assert!(!inner.is_null());
        assert!(!inner.is_undefined());
        Xos_Js_SharedArrayBuffer { inner }
    }

    pub fn to_jsvalue(&self) -> &wb::JsValue {
        self.inner.deref()
    }

    pub fn into_jsvalue(self) -> wb::JsValue {
        self.inner.into()
    }

    pub fn new_slice(s: &[u8]) -> Xos_Js_SharedArrayBuffer {
        let array_buffer = js_sys::SharedArrayBuffer::new(s.len() as u32);
        let u8_arr = js_sys::Uint8Array::new(&array_buffer);
        u8_arr.copy_from(s);
        Xos_Js_SharedArrayBuffer { inner: array_buffer }
    }

    pub fn new_size(n: u32) -> Xos_Js_SharedArrayBuffer {
        let array_buffer = js_sys::SharedArrayBuffer::new(n);
        Xos_Js_SharedArrayBuffer { inner: array_buffer }
    }
}

    fn write_to_js(
        &self,
        _JsData_Write,


        _transfers: &mut VecDeque<wb::JsValue>,
    ) -> Result<(), JsData_Err> {
        _non_transfers.push_back(self.inner.clone().into());
        Ok(())
    }

    fn read_from_js(
        _JsData_Read,

        _transfers: &mut VecDeque<wb::JsValue>,
    ) -> Result<Xos_Js_SharedArrayBuffer, JsData_Err> {
        let ab = _non_transfers.pop_front().unwrap();
        Ok(Xos_Js_SharedArrayBuffer::new_jsv(&ab))
    }

    fn write_to_buf(&self, _JsData_Write) -> Result<(), JsData_Err> {
        damn_it!("")
    }

    fn read_from_buf(_JsData_Read) -> Result<Self, JsData_Err>
    where
        Self: Sized,
    {
        damn_it!("")
    }
}


 */
