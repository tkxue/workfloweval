#[allow(unused_imports)]
use super::*;

/*
#[derive(Clone, Debug)]
pub struct Xos_Js_ArrayBuffer_Vec<T: Copy> {
    pub len: u32,
    pub inner: Xos_Js_ArrayBuffer,
    pub _t: PhantomData<T>,
}


impl<T: Copy> Xos_Js_ArrayBuffer_Vec<T> {
    pub fn new_slice(x: &[T]) -> Xos_Js_ArrayBuffer_Vec<T> {
        Xos_Js_ArrayBuffer_Vec {
            len: x.len() as u32,
            inner: Xos_Js_ArrayBuffer::new_slice(conv_bytes(x)),
            _t: Default::default(),
        }
    }
}

    fn write_to_js(&self, JsData_Write,  _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), JsData_Err>
    where
        Self: Sized,
    {
        self.len.write_to_js(writer,  _transfers)?;
        self.inner.write_to_js(writer,  _transfers)?;
        Ok(())
    }

    fn read_from_js(JsData_Read,  _transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, JsData_Err>
    where
        Self: Sized,
    {
        let len = u32::read_from_js(reader,  _transfers)?;
        let inner = Xos_Js_ArrayBuffer::read_from_js(reader,  _transfers)?;

        Ok(Xos_Js_ArrayBuffer_Vec {
            len,
            inner,
            _t: Default::default(),
        })
    }

    fn write_to_buf(&self, _JsData_Write) -> Result<(), JsData_Err> {
        damn_it!("Xos_Js_ArrayBuffer_Vec can not write to bin");
    }

    fn read_from_buf(_JsData_Read) -> Result<Self, JsData_Err>
    where
        Self: Sized,
    {
        damn_it!("Xos_Js_ArrayBuffer_Vec can not read from ");
    }
}


 */
