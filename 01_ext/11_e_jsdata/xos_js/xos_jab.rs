#[allow(unused_imports)]
use super::*;

pub struct Xos_Jab {
    pub inner: js_sys::ArrayBuffer,
}

impl Clone for Xos_Jab {
    fn clone(&self) -> Self {
        Xos_Jab { inner: self.inner.slice(0) }
    }
}

impl Debug for Xos_Jab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[Xos_Js_ArrayBuffer]]")
    }
}

impl Xos_Jab {
    pub fn new_jsv(x: &wb::JsValue) -> Xos_Jab {
        let inner = js_sys::ArrayBuffer::from(x.clone());
        assert!(!inner.is_null());
        assert!(!inner.is_undefined());
        Xos_Jab { inner }
    }

    pub fn new_slice(s: &[u8]) -> Xos_Jab {
        let array_buffer = js_sys::ArrayBuffer::new(s.len() as u32);
        let u8_arr = js_sys::Uint8Array::new(&array_buffer);
        u8_arr.copy_from(s);
        Xos_Jab { inner: array_buffer }
    }

    pub fn new_slice_n(s: &[u8], n: usize) -> Xos_Jab {
        let array_buffer = js_sys::ArrayBuffer::new(n as u32);
        let u8_arr = js_sys::Uint8Array::new(&array_buffer);
        u8_arr.subarray(0, s.len() as u32).copy_from(s);
        Xos_Jab { inner: array_buffer }
    }

    pub fn bytes_len(&self) -> usize {
        self.inner.byte_length() as usize
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = vec![0_u8; self.inner.byte_length() as usize];
        let u8_arr = js_sys::Uint8Array::new(&self.inner);
        u8_arr.copy_to(v.as_mut_slice());
        v
    }

    pub fn to_vec_n0(&self, n: usize) -> Vec<u8> {
        let bl = self.inner.byte_length() as usize;
        let mut v = vec![0_u8; bl.max(n)];
        let u8_arr = js_sys::Uint8Array::new(&self.inner);
        u8_arr.copy_to(&mut v.as_mut_slice()[0..bl]);
        v
    }

    pub fn to_vec_part(&self, start: usize, len: usize) -> Vec<u8> {
        let mut v = vec![0_u8; len];

        let u8_arr = js_sys::Uint8Array::new_with_byte_offset_and_length(&self.inner, start as u32, len as u32);

        u8_arr.copy_to(v.as_mut_slice());
        v
    }

    pub fn to_jsvalue(&self) -> &wb::JsValue {
        self.inner.deref()
    }

    pub fn into_jsvalue(self) -> wb::JsValue {
        self.inner.into()
    }
}

impl T_JsData_ for wb::JsValue {
    fn write_to_js(&self, _writer: T_JsData_Write, transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        transfers.push_back(self.into());
        Ok(())
    }

    fn read_from_js(_reader: T_JsData_Read, transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        let ab = transfers.pop_front().unwrap();
        Ok(ab)
    }
}

impl T_JsData_ for Xos_Jab {
    fn write_to_js(&self, _: T_JsData_Write, transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        transfers.push_back(self.inner.clone().into());
        Ok(())
    }

    fn read_from_js(_: T_JsData_Read, transfers: &mut VecDeque<wb::JsValue>) -> Result<Xos_Jab, L_JsData_Err> {
        let ab = transfers.pop_front().unwrap();
        Ok(Xos_Jab::new_jsv(&ab))
    }
}
