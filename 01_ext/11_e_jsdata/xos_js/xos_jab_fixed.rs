use super::*;

#[derive(Clone)]
pub struct Xos_Jab_Fixed<const N: usize> {
    pub inner: js_sys::ArrayBuffer,
}

impl<const N: usize> Debug for Xos_Jab_Fixed<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[Xos_Js_ArrayBuffer]]")
    }
}

impl<const N: usize> Xos_Jab_Fixed<N> {
    pub fn new_jsv(x: &wb::JsValue) -> Xos_Jab_Fixed<N> {
        let inner = js_sys::ArrayBuffer::from(x.clone());
        assert!(!inner.is_null());
        assert!(!inner.is_undefined());
        Xos_Jab_Fixed { inner }
    }

    pub fn new_slice(s: &[u8]) -> Result<Xos_Jab_Fixed<N>, String> {
        if s.len() != N {
            return Err(format!("Xos_Jag_Fixed: need: {:?}, got: {:?}", N, s.len()));
        }
        let array_buffer = js_sys::ArrayBuffer::new(s.len() as u32);
        let u8_arr = js_sys::Uint8Array::new(&array_buffer);
        u8_arr.copy_from(s);
        Ok(Xos_Jab_Fixed { inner: array_buffer })
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

impl<const N: usize> T_JsData_ for Xos_Jab_Fixed<N> {
    fn write_to_js(&self, _: T_JsData_Write, transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        transfers.push_back(self.inner.clone().into());
        Ok(())
    }

    fn read_from_js(_: T_JsData_Read, transfers: &mut VecDeque<wb::JsValue>) -> Result<Xos_Jab_Fixed<N>, L_JsData_Err> {
        let ab = transfers.pop_front().unwrap();
        Ok(Xos_Jab_Fixed::new_jsv(&ab))
    }
}
