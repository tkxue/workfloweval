#[allow(unused_imports)]
use super::*;

/*
#[derive(Clone)]
pub struct Xos_Js_Uint32Array {
    inner: js_sys::Uint32Array,}

impl Xos_Js_Uint32Array {
    pub fn new_jsv(x: &wb::JsValue) -> Xos_Js_Uint32Array {
        let inner = js_sys::Uint32Array::new_with_byte_offset(&x, 0);
        Xos_Js_Uint32Array { inner}}

    pub fn to_js_array_buffer(&self) -> Xos_Js_ArrayBuffer {
        Xos_Js_ArrayBuffer::new_jsv(self.inner.buffer().deref())}

    pub fn to_js_value(&self) -> wb::JsValue {
        self.to_js_array_buffer().into_jsvalue()}

    pub fn to_vec(&self) -> Vec<u32> {
        let mut file_contents = vec![0; self.inner.length() as usize];
        self.copy_to(file_contents.as_mut_slice());
        file_contents}

    fn copy_to(&self, dst: &mut [u32]) {
        / let u32_arr = js_sys::Uint32Array::new(&self.inner);
        assert_eq!(self.inner.length(), dst.len() as u32);
        self.inner.copy_to(dst);}

    pub fn new_slice(d: &[u32]) -> Xos_Js_Uint32Array {
        let array_buffer = Self::new_len(d.len());
        array_buffer.copy_from(d);
        array_buffer}

    pub fn new_len(n: usize) -> Xos_Js_Uint32Array {
        Xos_Js_Uint32Array {
            inner: js_sys::Uint32Array::new_with_length(n as u32),}}

    fn copy_from(&self, dst: &[u32]) {
        self.inner.copy_from(dst);}}


 */
