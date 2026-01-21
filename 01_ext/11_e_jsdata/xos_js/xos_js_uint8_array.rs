#[allow(unused_imports)]
use super::*;

/*
#[derive(Clone)]
pub struct Xos_Js_Uint8Array {
    inner: js_sys::Uint8Array,}

impl Xos_Js_Uint8Array {
    pub fn new_jsv(x: &wb::JsValue) -> Xos_Js_Uint8Array {
        let inner = js_sys::Uint8Array::new_with_byte_offset(&x, 0);
        Xos_Js_Uint8Array { inner}}

    pub fn new_len(n: usize) -> Xos_Js_Uint8Array {
        Xos_Js_Uint8Array {
            inner: js_sys::Uint8Array::new_with_length(n as u32),}}

    pub fn new_slice(d: &[u8]) -> Xos_Js_Uint8Array {
        let array_buffer = Self::new_len(d.len());
        array_buffer.copy_from(d);
        array_buffer}

    fn copy_from(&self, dst: &[u8]) {
        assert!(!self.inner.is_null());
        assert!(!self.inner.is_undefined());
        self.inner.copy_from(dst);}

    pub fn region_copy_from(&self, start: usize, length: usize, dst: &[u8]) {
        assert!(!self.inner.is_null());
        assert!(!self.inner.is_undefined());
        let u8_arr = js_sys::Uint8Array::new_with_byte_offset_and_length(
            &self.inner.buffer(),
            start as u32,
            length as u32,);
        u8_arr.copy_from(dst);}

    pub fn to_ab(&self) -> Xos_Js_ArrayBuffer {
        assert!(!self.inner.is_null());
        assert!(!self.inner.is_undefined());
        Xos_Js_ArrayBuffer::new_jsv(self.inner.buffer().deref())}

    pub fn to_js_array_buffer(&self) -> Xos_Js_ArrayBuffer {
        Xos_Js_ArrayBuffer::new_jsv(self.inner.buffer().deref())}

    pub fn to_js_value(&self) -> wb::JsValue {
        self.to_js_array_buffer().into_jsvalue()}

    pub fn to_vec(&self) -> Vec<u8> {
        assert!(!self.inner.is_null());
        assert!(!self.inner.is_undefined());
        let mut file_contents = vec![0; self.inner.length() as usize];
        self.copy_to(file_contents.as_mut_slice());
        file_contents}

    fn copy_to(&self, dst: &mut [u8]) {
        assert!(!self.inner.is_null());
        assert!(!self.inner.is_undefined());
        / let u8_arr = js_sys::Uint8Array::new(&self.inner);
        assert_eq!(self.inner.length(), dst.len() as u32);
        self.inner.copy_to(dst);}}


 */
