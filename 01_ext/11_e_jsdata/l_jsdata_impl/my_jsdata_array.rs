#[allow(unused_imports)]
use super::*;
use std::mem;
use std::mem::MaybeUninit;

unsafe fn maybe_uninit_slice__to__slice<const N: usize, T: Sized>(x: [MaybeUninit<T>; N]) -> [T; N] {
    let mut x = x;
    let ptr = &mut x as *mut _ as *mut [T; N];
    let res = unsafe { ptr.read() };
    core::mem::forget(x);
    res
}

/* writer: JsData_Write, reader: JsData_Read , */

impl<const N: usize, T: T_JsData_ + Sized> T_JsData_ for [T; N] {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        for i in 0..N {
            T::write_to_js(&self[i], writer, _transfers)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        let mut out: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            let t = T::read_from_js(reader, _transfers)?;
            out[i].write(t);
        }
        Ok(unsafe { maybe_uninit_slice__to__slice(out) })
    }
}
